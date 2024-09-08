
public class ImportServer {
    public static ImportServer Create() {
        return new ImportServer();
    }

    internal void Run() {
        Console.WriteLine("server is running!");
        Thread.Sleep(10000);
    }
}
