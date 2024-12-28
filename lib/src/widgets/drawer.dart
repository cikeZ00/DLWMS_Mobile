import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:dlwms_mobile/src/pages/login.dart';
import 'package:dlwms_mobile/src/pages/home.dart';
import 'package:dlwms_mobile/src/pages/webview_page.dart';

class GlobalDrawer extends StatelessWidget {
  const GlobalDrawer({super.key});

  Future<void> _logout(BuildContext context) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.remove('cookies');
    Navigator.pushReplacement(
      context,
      MaterialPageRoute(builder: (context) => const LoginPage()),
    );
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Drawer(
      child: Column(
        children: <Widget>[
          DrawerHeader(
            decoration: const BoxDecoration(
              color: Colors.white,
            ),
            child: Center(
              child: Image.asset(
                'assets/icon/icon.png',
                width: 300,
                height: 300,
              ),
            ),
          ),
          ListTile(
            title: Text('Home', style: theme.textTheme.bodyLarge),
            onTap: () {
              Navigator.pushReplacement(
                context,
                MaterialPageRoute(builder: (context) => const MyHomePage()),
              );
            },
          ),
          ListTile(
            title: Text('Student Portal', style: theme.textTheme.bodyLarge),
            onTap: () {
              Navigator.push(
                context,
                MaterialPageRoute(builder: (context) => const WebViewPage()),
              );
            },
          ),
          const Spacer(),
          Align(
            alignment: Alignment.bottomCenter,
            child: ListTile(
              title: Text('Logout', style: theme.textTheme.bodyLarge),
              onTap: () => _logout(context),
            ),
          ),
        ],
      ),
    );
  }
}