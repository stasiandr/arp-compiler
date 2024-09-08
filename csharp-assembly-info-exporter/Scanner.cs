using System.Reflection;
using System.Text.Json;
using System.Text.RegularExpressions;

public static class Scanner
{

    internal static ArpTypeCollection ScanDll(string path)
    {
        var collection = new ArpTypeCollection();

        var asm = Assembly.LoadFile(Path.GetFullPath(path));

        foreach (var ty in asm.GetExportedTypes())
        {
            try
            {
                collection.PushType(ty);    
            }
            catch (System.Exception e)
            {
                // For debug only
                Console.WriteLine(e);
            }
        }

        return collection;
    }

    public static string ScanDllToJson(string path)
    {
        return JsonSerializer.Serialize(ScanDll(path), new JsonSerializerOptions
        {
            PropertyNamingPolicy = new SnakeCaseNamingPolicy(),
            WriteIndented = false
        });
    }

    public class SnakeCaseNamingPolicy : JsonNamingPolicy
{
    public override string ConvertName(string name)
    {
        // Converts PascalCase or camelCase to snake_case
        return Regex.Replace(name, "(?<!^)([A-Z])", "_$1").ToLower();
    }
}
}