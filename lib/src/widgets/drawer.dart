import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:dlwms_mobile/src/pages/login.dart';
import 'package:dlwms_mobile/src/pages/home.dart';

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
            child: Center(
              child: Container(
                decoration: BoxDecoration(
                  color: theme.colorScheme.primary,
                  borderRadius: BorderRadius.circular(8.0),
                ),
                child: Padding(
                  // Add padding to the size of the drawer header
                  padding: const EdgeInsets.all(30.0),
                  child: Text(
                    'DLWMS',
                    style: theme.textTheme.displayLarge,
                  ),
                ),
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