use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

use crate::il_token::{FunctionFlags, ILToken, OpCode, StructureFlags};

pub fn write_tokens_to_file<P: AsRef<Path>>(path: &P, tokens: &[ILToken]) -> Result<(), io::Error> {
    let path = path.as_ref();
    if path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "The path is a directory, cannot write tokens.",
        ));
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // Prelude
    writeln!(writer, ".assembly 'Main' {{}}")?;

    write_tokens(&mut writer, tokens)?;

    writer.flush()?;

    Ok(())
}

fn write_tokens(writer: &mut BufWriter<File>, tokens: &[ILToken]) -> Result<(), io::Error> {
    for token in tokens {
        match token {
            ILToken::OpCode(opcode) => {
                write!(writer, "\t\t")?;
                write_op_code(writer, opcode)?;
            }
            ILToken::StartMethod(method) => {
                write_method(writer, method)?;
            }
            ILToken::EndMethod(mtd_name) => {
                writeln!(writer, "\t\tret")?;
                writeln!(writer, "\t}} // {mtd_name}\n")?
            }
            ILToken::Empty => writeln!(writer)?,

            ILToken::StartStructure(flags, name) => {
                write!(writer, "\n.class ")?;
                if flags.contains(&StructureFlags::Auto) {
                    write!(writer, "auto ")?;
                }

                writeln!(writer, "{name}")?;

                writeln!(writer, "{{")?;

            }
            ILToken::EndStructure(name) => writeln!(writer, "}} // {name}\n")?,
            ILToken::Field(name, ty) => writeln!(writer, "\t.field public {} {name}", ty.0)?,
        }
    }

    Ok(())
}

fn write_op_code(writer: &mut BufWriter<File>, opcode: &OpCode) -> Result<(), io::Error> {
    

    match opcode {
        OpCode::LoadInt(i) => writeln!(writer, "ldc.i4.s {}", i)?,
        OpCode::LoadFloat(f) => writeln!(writer, "ldc.r4 {}", f)?,
        OpCode::LoadString(s) => writeln!(writer, "ldstr \"{}\"", s)?,
        OpCode::LoadBool(b) => writeln!(writer, "ldc.i4.{}", {
            if *b {
                1
            } else {
                0
            }
        })?,
        OpCode::StoreLocalVariable(index) => writeln!(writer, "stloc.s {}", index)?,
        OpCode::LoadLocalVariable(index) => writeln!(writer, "ldloc.s {}", index)?,
        OpCode::Add => writeln!(writer, "add")?,
        OpCode::Multiply => writeln!(writer, "mul")?,
        OpCode::Subtract => writeln!(writer, "sub")?,
        OpCode::Divide => writeln!(writer, "div")?,
        OpCode::Equal => writeln!(writer, "ceq")?,
        OpCode::LessThen => writeln!(writer, "clt")?,
        OpCode::GreaterThen => writeln!(writer, "cgt")?,
        OpCode::Or => writeln!(writer, "or")?,
        OpCode::And => writeln!(writer, "and")?,
        OpCode::Call {
            is_instance,
            return_type,
            external,
            ty,
            method_name,
            args,
        } => {
            write!(writer, "call ")?;
            if *is_instance {
                write!(writer, "instance ")?
            }
            write!(writer, "{} ", return_type.0)?;
            if let Some(external) = external {
                write!(writer, "[{}]", external)?
            }
            write!(writer, "{}::{}", ty, method_name)?;
            
            writeln!(
                writer,
                "({})",
                args.iter()
                .map(|arg| arg.0.clone())
                .collect::<Vec<_>>()
                .join(",")
            )?;
        }
        OpCode::NoOperation => writeln!(writer, "nop")?,
        OpCode::NewObject(ty, args) => writeln!(
            writer,
            "newobj instance void {}::.ctor({})",
            ty.0,
            args.iter()
            .map(|arg| arg.0.clone())
            .collect::<Vec<_>>()
            .join(",")
        )?,
        OpCode::SetField(fld_ty, class_name, fld_name) => {
            writeln!(writer, "stfld {} {}::{}", fld_ty.0, class_name, fld_name)?; 
        },
        OpCode::GetField(fld_ty, class_name, fld_name) => { 
            writeln!(writer, "ldfld {} {}::{}", fld_ty.0, class_name, fld_name)?;
        }
        OpCode::LoadArgument(index) => writeln!(writer, "ldarg.s {}", index)?,
        OpCode::StoreArgument(index) => writeln!(writer, "starg.s {}", index)?,

        // IL_0009: brfalse.s IL_0016
        OpCode::BranchIfFalse(label) => writeln!(writer, "brfalse.s {}", label)?,
        OpCode::BranchIfTrue(label) => writeln!(writer, "brtrue.s {}", label)?,
        OpCode::BranchTo(index) => writeln!(writer, "br.s {}", index)?,
        OpCode::LabeledOpCode(label, opcode) => {
            write!(writer, "{}:", label)?;
            write_op_code(writer, opcode)?;
        },
        OpCode::Return => writeln!(writer, "ret")?,
    };

    Ok(())
}

fn write_method(
    writer: &mut BufWriter<File>,
    method: &crate::il_token::Method,
) -> Result<(), io::Error> {
    write!(writer, "\n\t.method ")?;
    if method.flags.contains(&FunctionFlags::IsStatic(true)) {
        write!(writer, "static ")?;
    } else if method.flags.contains(&FunctionFlags::IsStatic(false)) {
        write!(writer, "instance ")?;
    } else {
        panic!();
    }
    write!(writer, "{} ", method.return_ty.0)?;
    write!(writer, "{} ", method.name)?;
    write!(writer, "(")?;


    write!(writer, "{}", method.params.iter()
        .map(|(name, ty)| format!("{} {}", ty.0, name))
        .collect::<Vec<_>>()
        .join(",\n"))?;

    write!(writer, ") ")?;
    if method.flags.contains(&FunctionFlags::Cil) {
        write!(writer, "cil ")?;
    }
    if method.flags.contains(&FunctionFlags::Managed) {
        write!(writer, "managed ")?;
    }
    writeln!(writer)?;
    writeln!(writer, "\t{{")?;
    if method.flags.contains(&FunctionFlags::EntryPoint) {
        writeln!(writer, "\t\t.entrypoint")?;
    }
    writeln!(writer, "\t\t.locals init (")?;

    writeln!(
        writer,
        "{}",
        method
            .registers
            .iter()
            .enumerate()
            .map(|(index, ty)| format!("\t\t\t[{}] {}", index, ty.0))
            .collect::<Vec<_>>()
            .join(",\n")
    )?;

    writeln!(writer, "\t\t)")?;
    Ok(())
}
