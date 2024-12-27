import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:dlwms_mobile/src/rust/api/simple.dart';
import 'package:dlwms_mobile/src/widgets/drawer.dart';

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key});

  @override
  _MyHomePageState createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  String _fetchedData = '';
  String _cookies = '';
  List<News> _newsList = [];

  @override
  void initState() {
    super.initState();
    _loadCookies();
  }

  Future<void> _loadCookies() async {
    final prefs = await SharedPreferences.getInstance();
    setState(() {
      _cookies = prefs.getString('cookies') ?? '';
    });
    _fetchData(_cookies);
  }

  Future<void> _fetchData(String cookies) async {
    try {
      final response = await requestHomeSync(cookies: cookies);
      final List<dynamic> newsJson = jsonDecode(response);
      final List<News> newsList = newsJson.map((json) => News.fromJson(json)).toList();
      setState(() {
        _newsList = newsList;
        _fetchedData = 'Home page requested successfully';
      });
    } catch (e) {
      setState(() {
        _fetchedData = 'Fetch failed: $e';
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Home'),
      ),
      drawer: const GlobalDrawer(),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: <Widget>[
            Text(_fetchedData),
            Expanded(
              child: ListView.builder(
                itemCount: _newsList.length,
                itemBuilder: (context, index) {
                  final news = _newsList[index];
                  return Card(
                    margin: const EdgeInsets.symmetric(vertical: 8.0),
                    child: Padding(
                      padding: const EdgeInsets.all(16.0),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            news.title,
                            style: Theme.of(context).textTheme.headlineSmall,
                          ),
                          const SizedBox(height: 8.0),
                          Text(
                            '${news.date} - ${news.author}',
                            style: Theme.of(context).textTheme.labelSmall,
                          ),
                          const SizedBox(height: 8.0),
                          Text(
                            news.subject,
                            style: Theme.of(context).textTheme.titleMedium  ,
                          ),
                          const SizedBox(height: 8.0),
                          Text(
                            news.abstractText,
                            style: Theme.of(context).textTheme.bodyLarge,
                          ),
                        ],
                      ),
                    ),
                  );
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class News {
  final String title;
  final String date;
  final String subject;
  final String author;
  final String abstractText;

  News({
    required this.title,
    required this.date,
    required this.subject,
    required this.author,
    required this.abstractText,
  });

  factory News.fromJson(Map<String, dynamic> json) {
    return News(
      title: json['title'],
      date: json['date'],
      subject: json['subject'],
      author: json['author'],
      abstractText: json['abstract_text'],
    );
  }
}