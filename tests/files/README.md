| Filename       | Build Options                                     | Compression |
|----------------|---------------------------------------------------|-------------|
| 0-uncompressed | `BuildAssetBundleOptions.UncompressedAssetBundle` | None        |
| 1-default      | `BuildAssetBundleOptions.None`                    | lzma        |
| 2-chunkbased   | `BuildAssetBundleOptions.ChunkBasedCompression`   | lz4hc       |


```cs
public static class Editor {
    private const string TargetDir = "Assets/AssetBundles/all";

    [MenuItem("TEST/Build All")]
    private static void BuildAll() {
        Directory.Delete(TargetDir);
        Directory.CreateDirectory(TargetDir);

        Build("uncompressed", BuildAssetBundleOptions.UncompressedAssetBundle);
        Build("default", BuildAssetBundleOptions.None);
        Build("chunkbased", BuildAssetBundleOptions.ChunkBasedCompression);
    }

    private static void Build(string name, BuildAssetBundleOptions options) {
        BuildPipeline.BuildAssetBundles("Assets/AssetBundles",
            options | BuildAssetBundleOptions.ForceRebuildAssetBundle | BuildAssetBundleOptions.StrictMode,
            BuildTarget.StandaloneLinux64);

        var target = $"{TargetDir}/{name}-{Application.unityVersion}.bundle";
        File.Move("Assets/AssetBundles/testassetbundle", target);
    }
}
```