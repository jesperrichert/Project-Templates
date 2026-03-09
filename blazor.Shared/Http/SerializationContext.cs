using System.Text.Json;
using System.Text.Json.Serialization;
using blazor.Shared.Http.Request.Auth;
using blazor.Shared.Http.Request.User;

namespace blazor.Shared.Http;

[JsonSerializable(typeof(CreateUserDto))]
[JsonSerializable(typeof(UpdateUserDto))]
[JsonSerializable(typeof(ClaimDto[]))]
[JsonSerializable(typeof(SchemeDto[]))]

[JsonSourceGenerationOptions(JsonSerializerDefaults.Web)]
public partial class SerializationContext : JsonSerializerContext
{
    
}