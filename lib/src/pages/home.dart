import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:dlwms_mobile/src/rust/api/simple.dart';
import 'news_details.dart';
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
  int _currentPage = 1;
  bool _isLoading = false;

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
    _fetchData(_cookies, _currentPage);
  }

  Future<void> _fetchData(String cookies, int page) async {
    if (_isLoading) return;
    setState(() {
      _isLoading = true;
    });

    try {
      final response = await requestHomeSync(cookies: cookies, pageIndex: BigInt.from(page));
      final List<dynamic> newsJson = jsonDecode(response);
      final List<News> newsList = newsJson.map((json) => News.fromJson(json)).toList();
      setState(() {
        _newsList.addAll(newsList);
        _currentPage++;
      });
    } catch (e) {
      setState(() {
        _fetchedData = 'Fetch failed: $e';
      });
    } finally {
      setState(() {
        _isLoading = false;
      });
    }
  }

  void _openNewsDetails(String url) {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => NewsDetailsPage(url: url, cookies: _cookies),
      ),
    );
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
            if (_fetchedData.isNotEmpty) Text(_fetchedData),
            Expanded(
              child: NotificationListener<ScrollNotification>(
                onNotification: (ScrollNotification scrollInfo) {
                  if (scrollInfo.metrics.pixels == scrollInfo.metrics.maxScrollExtent && !_isLoading) {
                    _fetchData(_cookies, _currentPage);
                  }
                  return false;
                },
                child: ListView.builder(
                  itemCount: _newsList.length,
                  itemBuilder: (context, index) {
                    final news = _newsList[index];
                    return GestureDetector(
                      onTap: () => _openNewsDetails(news.link),
                      child: Card(
                        margin: const EdgeInsets.symmetric(vertical: 4.0),
                        child: Padding(
                          padding: const EdgeInsets.all(8.0),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(news.title, style: Theme.of(context).textTheme.headlineSmall),
                              const SizedBox(height: 4.0),
                              Text('${news.date}  ${news.author}', style: Theme.of(context).textTheme.labelSmall),
                              const SizedBox(height: 4.0),
                              Text(news.subject, style: Theme.of(context).textTheme.titleMedium),
                              const SizedBox(height: 2.0),
                              Text(news.abstractText, style: Theme.of(context).textTheme.bodyMedium),
                            ],
                          ),
                        ),
                      ),
                    );
                  },
                ),
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
  final String link;

  News({
    required this.title,
    required this.date,
    required this.subject,
    required this.author,
    required this.abstractText,
    required this.link,
  });

  factory News.fromJson(Map<String, dynamic> json) {
    return News(
      title: json['title'],
      date: json['date'],
      subject: json['subject'],
      author: json['author'],
      abstractText: json['abstract_text'],
      link: json['link'],
    );
  }
}
