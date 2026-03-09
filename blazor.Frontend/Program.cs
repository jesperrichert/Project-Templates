using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using blazor.Frontend;
using blazor.Frontend.Startup;
using blazor.Frontend.UI;
using ShadcnBlazor;
using ShadcnBlazor.Extras;

public static class Program {

    public static async Task Main(String[] args)
    {
        var builder = WebAssemblyHostBuilder.CreateDefault(args);
        
        builder.PerformPreBuild();

        await builder.Build().RunAsync();
    }
    
}