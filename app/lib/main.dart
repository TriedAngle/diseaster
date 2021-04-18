import 'package:flutter/material.dart';
import 'dart:async';
import 'dart:convert';

import 'package:speech_to_text/speech_to_text.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart' as DotEnv;
import 'package:speech_to_text/speech_recognition_result.dart';
import 'package:speech_to_text/speech_recognition_error.dart';
import 'package:http/http.dart' as http;

Future main() async {
  await DotEnv.load(fileName: ".env");
  runApp(MyApp());
}

class MyApp extends StatefulWidget {
  @override
  _MyAppState createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  TextEditingController controller;
  bool _hasSpeech = false;
  bool sending = false;
  String lastWords = "";
  String lastError = "";
  String lastStatus = "";
  String responseText = "";
  final SpeechToText speech = SpeechToText();

  @override
  void initState() {
    super.initState();
    controller = TextEditingController();
    initSpeechState();
  }

  @override
  void dispose() {
    controller.dispose();
    super.dispose();
  }

  Future<void> initSpeechState() async {
    bool hasSpeech = await speech.initialize(
        onError: errorListener, onStatus: statusListener);

    if (!mounted) return;
    setState(() {
      _hasSpeech = hasSpeech;
    });
  }

  void sendText() async {
    setState(() {
      sending = true;
    });
    var uri = Uri.parse(DotEnv.env["MATCH_SERVICE_ADDRESS"] + "/api/matcher");
    var body = {'text': lastWords};
    print(body);
    var response = await http.post(uri, body: body);
    if (response.statusCode == 200) {
      var decode = json.decode(response.body);
      setState(() {
        responseText = decode['text'];
      });
    }
    setState(() {
      sending = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Diseaster'),
        ),
        body: _hasSpeech
            ? Column(children: [
                Expanded(
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: <Widget>[
                      TextButton(
                        child: Text('Start'),
                        onPressed: startListening,
                      ),
                      TextButton(
                        child: Text('Stop'),
                        onPressed: stopListening,
                      ),
                      TextButton(
                        child: Text('Cancel'),
                        onPressed: cancelListening,
                      ),
                    ],
                  ),
                ),
                Expanded(
                  child: Column(
                    children: <Widget>[
                      Center(
                        child: Text(
                          'Text:',
                          style: TextStyle(
                              fontSize: 17.0, color: Colors.blueAccent),
                        ),
                      ),
                      Expanded(
                          child: TextField(
                            controller: controller,
                            onSubmitted: submitText,

                          )
                      )
                    ],
                  ),
                ),
                Expanded(
                  child: Column(
                    children: <Widget>[
                      Center(
                        child: Text(
                          'Response:',
                          style: TextStyle(
                              fontSize: 17.0, color: Colors.blueAccent),
                        ),
                      ),
                      Expanded(
                          child: Container(
                        child: responseText == "" ?
                          Text("Awaiting Input")
                            : Text(responseText)
                      ))
                    ],
                  ),
                ),
                Expanded(
                  child: Center(
                    child: TextButton(
                      child: Text('Send'),
                      onPressed: sendText,
                    ),
                  ),
                ),
                Expanded(
                  child: Center(
                    child: speech.isListening
                        ? Text("I'm listening...")
                        : Text('Not listening'),
                  ),
                ),
              ])
            : Center(
                child: Text('Speech recognition unavailable',
                    style: TextStyle(
                        fontSize: 20.0, fontWeight: FontWeight.bold))),
      ),
    );
  }

  void submitText(String text) {
    setState(() {
      lastWords = text;
    });
  }

  void startListening() {
    lastWords = "";
    lastError = "";
    speech.listen(onResult: resultListener);
    setState(() {});
  }

  void stopListening() {
    speech.stop();
    setState(() {});
  }

  void cancelListening() {
    speech.cancel();
    setState(() {});
  }

  void resultListener(SpeechRecognitionResult result) {
    final newValue = "${result.recognizedWords}";
    setState(() {
      lastWords = newValue;
    });

    controller.text = lastWords;
  }

  void errorListener(SpeechRecognitionError error) {
    setState(() {
      lastError = "${error.errorMsg} - ${error.permanent}";
    });
  }

  void statusListener(String status) {
    setState(() {
      lastStatus = "$status";
    });
  }
}
