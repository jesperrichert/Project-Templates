FROM mcr.microsoft.com/dotnet/aspnet:10.0 AS base
USER $APP_UID
WORKDIR /app
EXPOSE 8080
EXPOSE 8081

FROM mcr.microsoft.com/dotnet/sdk:10.0 AS build
ARG BUILD_CONFIGURATION=Release
WORKDIR /src
COPY ["csapitemplate.Api/csapitemplate.Api.csproj", "csapitemplate.Api/"]
RUN dotnet restore "csapitemplate.Api/csapitemplate.Api.csproj"
COPY . .
WORKDIR "/src/csapitemplate.Api"
RUN dotnet build "./csapitemplate.Api.csproj" -c $BUILD_CONFIGURATION -o /app/build

FROM build AS publish
ARG BUILD_CONFIGURATION=Release
RUN dotnet publish "./csapitemplate.Api.csproj" -c $BUILD_CONFIGURATION -o /app/publish /p:UseAppHost=false

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "csapitemplate.Api.dll"]
