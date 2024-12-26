// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.7.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These functions are ignored because they are not marked as `pub`: `extract_hidden_value`, `login`, `request_page`

Future<LoginResponse> loginSync(
        {required String username,
        required String password,
        required String institute}) =>
    RustLib.instance.api.crateApiSimpleLoginSync(
        username: username, password: password, institute: institute);

Future<PageRequestResponse> requestPageSync(
        {required String url, required String cookies}) =>
    RustLib.instance.api
        .crateApiSimpleRequestPageSync(url: url, cookies: cookies);

class LoginResponse {
  final bool success;
  final String message;
  final String? cookies;

  const LoginResponse({
    required this.success,
    required this.message,
    this.cookies,
  });

  @override
  int get hashCode => success.hashCode ^ message.hashCode ^ cookies.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LoginResponse &&
          runtimeType == other.runtimeType &&
          success == other.success &&
          message == other.message &&
          cookies == other.cookies;
}

class PageRequestResponse {
  final bool success;
  final String message;
  final String page;

  const PageRequestResponse({
    required this.success,
    required this.message,
    required this.page,
  });

  @override
  int get hashCode => success.hashCode ^ message.hashCode ^ page.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is PageRequestResponse &&
          runtimeType == other.runtimeType &&
          success == other.success &&
          message == other.message &&
          page == other.page;
}
