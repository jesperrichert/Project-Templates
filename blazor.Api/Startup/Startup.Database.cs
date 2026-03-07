using csapitemplate.Api.Configuration;
using csapitemplate.Api.Database;

namespace csapitemplate.Api.Startup;

public partial class Startup
{
    private static void AddDatabase(WebApplicationBuilder builder)
    {
        builder.Services.AddOptions<DatabaseOptions>().BindConfiguration("csapitemplate:Database");


        builder.Services.AddScoped(typeof(DatabaseRepository<>));
        builder.Services.AddDbContext<DataContext>();
    }
}