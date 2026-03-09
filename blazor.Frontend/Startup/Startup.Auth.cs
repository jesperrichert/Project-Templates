using blazor.Frontend.Services;
using blazor.Frontend.UI;
using Microsoft.AspNetCore.Components.Authorization;
using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using ShadcnBlazor;
using ShadcnBlazor.Extras;

namespace blazor.Frontend.Startup;

public static partial class Startup
{
    private static void AddAuth(WebAssemblyHostBuilder builder)
    {
        builder.Services.AddScoped<AuthenticationStateProvider, RemoteAuthProvider>();
        builder.Services.AddAuthorizationCore();
        builder.Services.AddCascadingAuthenticationState();
        
    }
}