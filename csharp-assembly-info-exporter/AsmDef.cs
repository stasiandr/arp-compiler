using System.Text.Json.Serialization;

public class ArpTypeCollection
{
    [JsonInclude]
    public List<ArpTypeInfo> collection = [];

    internal void PushType(Type ty)
    {
        var arpTypeInfo = new ArpTypeInfo
        {
            FullName = ty.FullName ?? throw new Exception("Can't determine type's full name."),
            ShortName = ty.Name,
        };

        foreach (var fld in ty.GetFields())
        {
            arpTypeInfo.Fields.Add(new ArpTypedIdent {
                Ident = fld.Name,
                TyFullName = fld.FieldType.FullName ?? throw new Exception("Can't determine type's full name."),
            });
        }

        foreach (var mtd in ty.GetMethods()){
            arpTypeInfo.Methods.Add(new ArpMethodInfo {
                Ident = mtd.Name,
                ReturnTyFullName = mtd.ReturnType.FullName ?? throw new Exception("Can't determine type's full name."),
                Args = mtd.GetParameters().Select(par => new ArpTypedIdent {
                    Ident = par.Name ?? "",
                    TyFullName = par.ParameterType.FullName ?? throw new Exception("Can't determine type's full name."),
                }).ToList(),
            });
        }

        collection.Add(arpTypeInfo);
    }
}

public class ArpTypeInfo
{
    
    [JsonInclude] public required string FullName;
    [JsonInclude] public string? ShortName;
    [JsonInclude] public List<ArpTypedIdent> Fields = [];
    [JsonInclude] public List<ArpMethodInfo> Methods = [];

}

public class ArpTypedIdent
{
    [JsonInclude] public required string Ident;
    [JsonInclude] public required string TyFullName;
}


public class ArpMethodInfo
{
    [JsonInclude] public required string Ident;
    [JsonInclude] public List<ArpTypedIdent> Args = [];
    [JsonInclude] public required string ReturnTyFullName;
}
