using System.Security.Claims;
using blazor.Api.Configuration;
using blazor.Api.Services;
using Microsoft.AspNetCore.Authentication;
using Microsoft.AspNetCore.Authentication.Cookies;
using Microsoft.AspNetCore.Authentication.OpenIdConnect;

namespace blazor.Api.Startup;

public partial class Startup
{
    private static void AddAuth(WebApplicationBuilder builder)
    {
        var oidcOptions = new OidcOptions();
        builder.Configuration.GetSection("blazor:Oidc").Bind(oidcOptions);

        builder.Services.AddScoped<UserAuthService>();

        builder.Services.AddAuthentication("Session")
            .AddCookie("Session", null, options =>
            {
                options.Events.OnSigningIn += async context =>
                {
                    var authService = context
                        .HttpContext
                        .RequestServices
                        .GetRequiredService<UserAuthService>();

                    var result = await authService.SyncAsync(context.Principal);

                    if (result)
                        context.Properties.IsPersistent = true;
                    else
                        context.Principal = new ClaimsPrincipal();
                };

                options.Events.OnValidatePrincipal += async context =>
                {
                    var authService = context
                        .HttpContext
                        .RequestServices
                        .GetRequiredService<UserAuthService>();

                    var result = await authService.ValidateAsync(context.Principal);

                    if (!result)
                        context.RejectPrincipal();
                };

                options.Cookie = new CookieBuilder()
                {
                    Name = "token",
                    Path = "/",
                    IsEssential = true,
                    SecurePolicy = CookieSecurePolicy.SameAsRequest
                };
            })
            .AddOpenIdConnect(OpenIdConnectDefaults.AuthenticationScheme, "OpenID Connect", options =>
            {
                var scopes = oidcOptions.Scopes ?? ["openid", "email", "profile"];
                options.Scope.Clear();

                foreach (var scope in scopes)
                {
                    options.Scope.Add(scope);
                }

                options.Authority = oidcOptions.Authority;
                options.RequireHttpsMetadata = oidcOptions.RequireHttpsMetadata;
                options.ClientId = oidcOptions.ClientId;
                options.ClientSecret = oidcOptions.ClientSecret;
                options.ResponseType = oidcOptions.ResponseType;

                options.ClaimActions.MapJsonKey(ClaimTypes.Name, "name");
                options.ClaimActions.MapJsonKey(ClaimTypes.Name, "preferred_username");
                options.ClaimActions.MapJsonKey(ClaimTypes.Email, "email");

                options.GetClaimsFromUserInfoEndpoint = true;
            });


        builder.Services.AddAuthorization();

        builder.Services.AddOptions<SessionsOptions>().BindConfiguration("blazor:Sessions");
    }

    private static void UseAuth(WebApplication application)
    {
        application.UseAuthentication();
        application.UseAuthorization();
    }
}