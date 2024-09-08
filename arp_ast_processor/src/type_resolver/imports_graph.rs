use std::collections::HashSet;

use crate::{errors::ProcessingError, types::{ast_node_value::{Ast, Id}, file::ArpFile, type_collection::TypeId}};

use super::{managed_dll_info, TypeResolverError};

#[derive(Default)]
pub struct ImportsGraph {

    root: NodeId,
    visited_paths: HashSet<String>,
    arp_files_cache: Vec<Id<ArpFile>>,

    nodes: Vec<ImportsGraphNode>
}

type NodeId = usize;

#[derive(Clone, Debug)]
pub struct ImportsGraphNode {
    pub file: Id<ArpFile>,
    pub import: Vec<(NodeId, Vec<TypeId>)>,
    pub extern_imports: Vec<(String, String)>
}



pub fn resolve_imports(mut ast: Ast) -> Result<Ast, ProcessingError> {
    let root_arp_file = *ast.get_children_of_kind::<ArpFile, _>(ast.get_root_index()).iter().find(|f| ast.get(f).arp_path.0 == "Main").unwrap();

    let graph = ImportsGraph::build_from(&ast, &root_arp_file)?;
    
    let mut nodes = graph.rec_iter().collect::<Vec<_>>();
    nodes.reverse();

    let types = nodes.iter().flat_map(|node| {
        node.import.iter().map(|(node_id, types)| {
            let types = types.iter()
                .map(|ty| ast
                    .get(&graph.get(*node_id).unwrap().file)
                    .type_collection
                    .try_get_strong(ty)
                    .unwrap().clone())
                .collect::<Vec<_>>();

            (node.file, ast.get(&graph.get(*node_id).unwrap().file).arp_path.clone(), types)
        })
    }).collect::<Vec<_>>();

    for (file, path, types) in types.into_iter().clone() {
        for ty in types {
            let import_to = ast.get_mut::<ArpFile>(&file);

            import_to.type_collection.copy_from(&ty, &path.0);
        }
    }

    for (file, external) in nodes.iter().map(|n| (n.file, n.extern_imports.clone())) {
        for (path, ty) in external {
            let import_to = ast.get_mut::<ArpFile>(&file);
            let sharp_type_info = managed_dll_info::resolve_type(&path, ty)?;
            import_to.type_collection.insert_external(path, &sharp_type_info);
        }
    }


    Ok(ast)
}


impl ImportsGraph {
    pub fn build_from(ast: &Ast, root_file: &Id<ArpFile>) -> Result<Self, TypeResolverError> {
        let mut graph = Self::default();
        graph.arp_files_cache.extend(ast.get_children_of_kind::<ArpFile, _>(ast.get_root_index()).iter());

        let node = graph.traverse(ast, root_file)?;
        graph.root = node;

        Ok(graph)
    }

    pub fn traverse(&mut self, ast: &Ast, file: &Id<ArpFile>) -> Result<NodeId, TypeResolverError> {
        let index = self.nodes.len();
        self.nodes.push(ImportsGraphNode {
            file: *file,
            import: vec![],
            extern_imports: vec![],
        });

        let file = ast.get(file);

        if self.visited_paths.contains(&file.arp_path.0) {
            return Err(TypeResolverError::RecursiveImportDetected(file.arp_path.0.clone()));
        } else {
            self.visited_paths.insert(file.arp_path.0.clone());
        }
        
        for import in file.imports.iter() {
            if import.is_extern {

                for ty in import.import_types.iter().map(|ident| ident.0.clone()) {
                    self.nodes[index].extern_imports.push((import.path.to_string(), ty.to_string()))    
                }

                continue;
            }

            let child_file = *self.arp_files_cache.iter()
                .find(|f| ast.get(f).arp_path.0 == import.path.as_ref())
                .ok_or(TypeResolverError::PathNotFound(import.path.to_string()))?;

            let child_node_id = self.traverse(ast, &child_file)?;

            let child_file = ast.get(&child_file);

            let new_import_types_collection = import.import_types.iter()
                .map(|import_type| child_file
                    .type_collection
                    .resolve_name(import_type)
                ).collect();
            
            self.nodes[index].import.push((child_node_id, new_import_types_collection));
        }

        Ok(index)
    }

    pub fn get(&self, index: usize) -> Option<&ImportsGraphNode> {
        self.nodes.get(index)
    }

    pub fn rec_iter(&self) -> ImportsGraphRecursiveIter<'_> {
        ImportsGraphRecursiveIter {
            graph: self,
            stack: vec![self.root],
        }
    }
}


pub struct ImportsGraphRecursiveIter<'a> {
    graph: &'a ImportsGraph,
    stack: Vec<NodeId>
}

impl<'a> Iterator for ImportsGraphRecursiveIter<'a> {
    type Item = &'a ImportsGraphNode;

    fn next(&mut self) -> Option<Self::Item> {

        if let Some(id) = self.stack.pop() {
            if let Some(node) = self.graph.nodes.get(id) {

                let children = node.import.iter().rev().map(|n| n.0);
                self.stack.extend(children);

                return Some(node);
            }
        }

        None
    }
}