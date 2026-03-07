using csapitemplate.Api.Configuration;
using Scalar.AspNetCore;

namespace csapitemplate.Api.Startup;

public partial class Startup
{
    private static void AddConfig(WebApplicationBuilder builder)
    {
        builder.Services.AddOptions<ApiOptions>().BindConfiguration("csapitemplate:Api");
    }
}