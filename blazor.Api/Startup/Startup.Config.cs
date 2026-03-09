using blazor.Api.Configuration;
using Scalar.AspNetCore;

namespace blazor.Api.Startup;

public partial class Startup
{
    private static void AddConfig(WebApplicationBuilder builder)
    {
        builder.Services.AddOptions<OidcOptions>().BindConfiguration("blazor:Oidc");
    }
}