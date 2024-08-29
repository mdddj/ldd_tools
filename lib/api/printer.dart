// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.3.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

///在打印数据中添加单色位图
Future<Uint8List> downloadBmpImageTsplCommandData(
        {required List<int> imageBuffer, required int x, required int y}) =>
    RustLib.instance.api.crateApiPrinterDownloadBmpImageTsplCommandData(
        imageBuffer: imageBuffer, x: x, y: y);

Future<Uint8List> tsplPrintCommandData() =>
    RustLib.instance.api.crateApiPrinterTsplPrintCommandData();
