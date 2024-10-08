// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.3.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'image.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<TsplCommandBuild>>
abstract class TsplCommandBuild implements RustOpaqueInterface {
  ///添加bmp图片
  /// [`image`] bmp图片资源
  /// [`pos`] x,y 位置
  Future<void> appendBmpImage(
      {required BitmapImage image, required (int, int) pos});

  ///添加数据
  void appendData({required List<int> data});

  ///获取字节数据
  Uint8List build();

  ///清除画布缓存
  /// 注意：该命令必须在SIZE命令之后
  void cls();

  /// 设置代码页及国际字符集,推荐UTF-8
  void codePage({required String n});

  ///自定义命令
  void command({required String command});

  ///设置浓度
  void density({required int n});

  ///定义两张标签之间的缝宽。单位毫米mm
  ///[`m`] 两个标签之间的距离
  ///[`n`] 缝的偏移
  ///0,0 连续纸
  void gap({required int m, required int n});

  /// 初始化
  factory TsplCommandBuild() =>
      RustLib.instance.api.crateApiTsplTsplCommandBuildNew();

  ///定义每次标签定位时可选择的、额外的标签进纸高度。主要在撕纸或切纸模式下，用于调整标签停止位置，并在下次打印前回退响应的距离。
  /// 单位: 毫米
  void offset({required int m});

  ///打印已缓存的标签。一般是(1,1)
  /// [`count`] 打印标签的数量,1: 打印标签的组数 2: 每组标签打印的数量
  void printer({required (int, int) count});

  ///移动标签的横向和纵向位置。正数使标签往打印方向的相反方向移动，负数使标签往打印方向移动。
  void shift({required int y});

  ///定义尺寸, 毫米类型
  void size({required (int, int) size});

  ///设置打印速度
  void speed({required int n});
}
