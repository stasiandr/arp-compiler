
using System.Reflection;

namespace csharp_assembly_info_exporter;

class Program {

    public static void Main(string[] args) {
        const int targetIndex = 0;

        if (args.Length > targetIndex) {
            if (args[targetIndex] == "--server") {
                var server = ImportServer.Create();
                server.Run();
            } else {
                var path = args[targetIndex];

                if (File.Exists(path)) {
                    var json = Scanner.ScanDllToJson(path);
                    Console.WriteLine(json);
                } else {
                    Console.WriteLine("File not found!");
                }
            }
        } else {
            Console.WriteLine("usage: either --server to start server or path to dll");
        }
    }
}