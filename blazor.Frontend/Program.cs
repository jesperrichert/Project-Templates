using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using blazor.Frontend;
using blazor.Frontend.UI;
using ShadcnBlazor;
using ShadcnBlazor.Extras;

var builder = WebAssemblyHostBuilder.CreateDefault(args);

builder.RootComponents.Add<App>("#app");
builder.RootComponents.Add<HeadOutlet>("head::after");

builder.Services.AddScoped(sp => new HttpClient { BaseAddress = new Uri(builder.HostEnvironment.BaseAddress) });

builder.Services.AddShadcnBlazor();
builder.Services.AddShadcnBlazorExtras();

await builder.Build().RunAsync();