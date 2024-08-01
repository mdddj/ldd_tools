# ldd_tools

Convert an image into an image that the printer can recognize

## Getting Started


add to your pubspec.yaml
```yaml
  ldd_tools: any
```


## init

`initLddToolLib()` add your main function

```dart
void main() {
  initLddToolLib(); // add this line
  runApp(const MyApp());
}
```



## Image thresholding

Convert images into image types that the printer can recognize

```dart
  Future<void> covertToImage(File file) async {
    try {
      final bfs = await file!.readAsBytes();
      bitmapImage = await lddCoverImageToLuma8(
          imageBuffer: bfs,
          thresholdValue: 128,
          width: 150,
          height: 150,
          thresholdType: LddThresholdType.binary,
          imageFormat: LddImageFormat.bmp);
      print(
          "cover result :${bitmapImage!.bitmap.length} ${bitmapImage!.width} ${bitmapImage!.height}");
      setState(() {});
    } catch (e) {
      print("err $e");
    }
  }
```

example

![](./images/image.png)