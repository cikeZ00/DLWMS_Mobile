import 'package:flutter/material.dart';

class ThemeProvider {
  static ThemeData get theme {
    return ThemeData(
      brightness: Brightness.dark,
      primaryColor: const Color(0xFFD81E5B),
      hintColor: const Color(0xFFD81E5B),
      scaffoldBackgroundColor: Colors.black,
      textTheme: const TextTheme(
        bodyLarge: TextStyle(color: Colors.white),
        bodyMedium: TextStyle(color: Colors.white70),
        displayLarge: TextStyle(color: Color(0xFFFDF0D5)),
        displayMedium: TextStyle(color: Color(0xFFFDF0D5)),
      ),
      colorScheme: ColorScheme.fromSwatch(
        primarySwatch: Colors.pink,
        brightness: Brightness.dark,
      ).copyWith(
        primary: const Color(0xFFD81E5B),
        secondary: const Color(0xFFD81E5B),
        surface: Colors.black,
        onSurface: Colors.white,
      ),
      // Add more theme customization here
    );
  }
}