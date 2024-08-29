import 'dart:io';
import 'dart:typed_data';

import 'package:dd_js_util/dd_js_util.dart';
import 'package:dd_js_util/model/models.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:ldd_tools/api/image.dart';
import 'package:ldd_tools/api/magick.dart';
import 'package:ldd_tools/ldd_tools.dart';

Future<void> main() async {
  await initLddToolLib();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  File? file;
  BitmapImage? bitmapImage;

  final tool = LddMagickTool();

  Uint8List? newImage;

  Future<void> selectFile() async {
    FilePickerResult? result = await FilePicker.platform.pickFiles();
    if (result != null) {
      file = File(result.files.single.path!);
      setState(() {});
    } else {
      // User canceled the picker
    }
  }

  ///转换图片
  Future<void> covertToImage(File file) async {
    setState(() {
      newImage = null;
    });
    final datas = await file.readAsBytes();
    final newData =
        await tool.covertToGrayColorImage(data: datas, format: "bmp");
    final finalData =
        await tool.coverToMonochrome(data: newData, format: "bmp");
    print('转换完成');
    newImage = finalData;
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('工具'),
        ),
        body: SingleChildScrollView(
          child: Column(
            children: [
              TextButton(onPressed: selectFile, child: const Text("选择一张图片")),
              if (file != null) Text(file!.path),
              if (file != null)
                SizedBox(
                  width: 100,
                  height: 100,
                  child: Image.file(file!),
                ),
              const SizedBox(height: 12),
              FilledButton(
                  onPressed: file == null ? null : () => covertToImage(file!),
                  child: const Text("图片转黑白单位色图")),
              const SizedBox(height: 12),
              if (bitmapImage != null)
                Image.memory(
                  bitmapImage!.bitmap,
                  width: bitmapImage!.width.toDouble(),
                  height: bitmapImage!.height.toDouble(),
                ),
              if (bitmapImage != null)
                Column(
                  children: [
                    Text(
                      "转换后图像大小:${ByteModel.create(bitmapImage!.bitmap.length.toDouble()).format(2)}",
                      style: context.textTheme.titleLarge,
                    ),
                    TextButton(
                        onPressed: () {
                          bitmapImage!.saveFile(path: "C:\\test111.bmp");
                        },
                        child: const Text('保存到文件'))
                  ],
                ),
              if (newImage != null)
                AspectRatio(aspectRatio: 1, child: Image.memory(newImage!))
            ],
          ),
        ),
      ),
    );
  }
}
