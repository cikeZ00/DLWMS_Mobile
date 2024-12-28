import 'package:flutter/material.dart';
import 'package:dlwms_mobile/src/rust/api/simple.dart';
import 'package:photo_view/photo_view.dart';
import 'dart:convert';
import 'dart:typed_data';

class NewsDetailsPage extends StatelessWidget {
  final String url;
  final String cookies;

  const NewsDetailsPage({required this.url, required this.cookies, super.key});

  Future<Map<String, dynamic>> _fetchNewsDetails() async {
    try {
      final response = await requestNewsSync(url: url, cookies: cookies);
      return jsonDecode(response);
    } catch (e) {
      print('Error fetching news details: $e');
      rethrow;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('News Details'),
      ),
      body: FutureBuilder<Map<String, dynamic>>(
        future: _fetchNewsDetails(),
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return const Center(child: CircularProgressIndicator());
          } else if (snapshot.hasError) {
            return Center(child: Text('Error: ${snapshot.error}'));
          } else if (!snapshot.hasData || snapshot.data!.isEmpty) {
            return const Center(child: Text('No data available'));
          } else {
            final newsDetails = snapshot.data!;
            final base64Image = newsDetails['file'];
            final tableData = newsDetails['table'] as List<dynamic>?;

            return SingleChildScrollView(
              padding: const EdgeInsets.all(16.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(newsDetails['title'], style: Theme.of(context).textTheme.headlineSmall),
                  const SizedBox(height: 8.0),
                  Text('${newsDetails['date']}  ${newsDetails['author']}', style: Theme.of(context).textTheme.labelSmall),
                  const SizedBox(height: 8.0),
                  Text(newsDetails['subject'], style: Theme.of(context).textTheme.titleMedium),
                  const SizedBox(height: 8.0),
                  Text(newsDetails['abstract_text'], style: Theme.of(context).textTheme.bodyLarge),
                  if (base64Image != null && base64Image.isNotEmpty) ...[
                    const SizedBox(height: 16.0),
                    GestureDetector(
                      onTap: () {
                        Navigator.push(
                          context,
                          MaterialPageRoute(
                            builder: (context) => ImageZoomView(imageBytes: base64Decode(base64Image)),
                          ),
                        );
                      },
                      child: Image.memory(base64Decode(base64Image)),
                    ),
                  ],
                  if (tableData != null && tableData.isNotEmpty) ...[
                    const SizedBox(height: 16.0),
                    ElevatedButton(
                      onPressed: () {
                        Navigator.push(
                          context,
                          MaterialPageRoute(
                            builder: (context) => TableViewPage(tableData: tableData.cast<List<dynamic>>()),
                          ),
                        );
                      },
                      child: const Text('View Table'),
                    ),
                  ],
                ],
              ),
            );
          }
        },
      ),
    );
  }
}

class ImageZoomView extends StatelessWidget {
  final Uint8List imageBytes;

  const ImageZoomView({required this.imageBytes, super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Image View'),
      ),
      body: Center(
        child: PhotoView(
          imageProvider: MemoryImage(imageBytes),
        ),
      ),
    );
  }
}

class TableViewPage extends StatelessWidget {
  final List<List<dynamic>> tableData;

  const TableViewPage({required this.tableData, super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Table View'),
      ),
      body: InteractiveViewer(
        constrained: false,
        boundaryMargin: const EdgeInsets.all(20.0),
        minScale: 0.1,
        maxScale: 5.0,
        child: SingleChildScrollView(
          scrollDirection: Axis.horizontal,
          child: DataTable(
            columns: tableData.first.map<DataColumn>((cell) {
              return DataColumn(label: Text(cell.toString()));
            }).toList(),
            rows: tableData.map<DataRow>((row) {
              return DataRow(
                cells: row.map<DataCell>((cell) {
                  return DataCell(Text(cell.toString()));
                }).toList(),
              );
            }).toList(),
          ),
        ),
      ),
    );
  }
}