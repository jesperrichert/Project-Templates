using System.Diagnostics.CodeAnalysis;
using blazor.Shared.Http.Request.User;
using blazor.Shared.Http.Response.User;
using Riok.Mapperly.Abstractions;

namespace blazor.Frontend.Mappers;

[Mapper]
[SuppressMessage("Mapper", "RMG020:No members are mapped in an object mapping")]
[SuppressMessage("Mapper", "RMG012:No members are mapped in an object mapping")]
public static partial class UserMapper
{
    public static partial UpdateUserDto ToUpdate(UserDto dto);
}