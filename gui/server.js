const express = require('express');
const fs = require('fs');
const { spawn, execFile } = require('child_process');
require('dotenv').config();

const app = express();
const PORT = process.env.PORT || 80;
const engine_path = process.env.ENGINE_PATH || "../engine/target/release/engine"

app.use(express.json());

let engineProcess = spawn(engine_path);

engineProcess.stdout.on('data', (data) => {
    console.log(`Engine Response: ${data}`);
});

// Handle errors from the engine
engineProcess.stderr.on('data', (data) => {
    console.error(`Engine Error: ${data}`);
});

app.get('/shutdown-engine', (req, res) => {
  if (req.query.key === '645312') {
    res.send('Shutting down Mac processes... You can close this tab.');
    console.log("Shutdown signal received. Closing...");
    
    // This tells the Node process to exit after 1 second
    setTimeout(() => { execFile("/Users/belalahmed/Documents/chessss/stop.command") }, 1000);
  } else {
    res.status(403).send('Forbidden');
  }
});

app.post("/move", async (req, res) => {
  const { moves, color, depth, fen_str } = req.body;
  try {
    engineProcess.stdin.write(JSON.stringify({command: "eval_move", moves: moves, color: color, depth: depth, fen_str: fen_str}) + "\n");

    res.json(await new Promise((resolve, reject) => {
      engineProcess.stdout.once('data', (data) => {
        data = data.toString();
        data = data.split(" ");
        resolve({move: data[0], evaluation: data[1]});
      })
    }))

    res.end();
    
  } catch (error) {
   console.log("Error Writing to stdin:", error);
    res.status(500).send(error.message);
  }
});

app.all(/.*/,(request,response)=>{
  request.url = (request.url == "/") ? "/index.html":request.url
  var questionofset = request.url.lastIndexOf("?")
  request.url = (questionofset == -1)?request.url:request.url.slice(0,questionofset)

  fs.readFile('.' + request.url, function(err, data) {
      if (!err) {
        var dotoffset = request.url.lastIndexOf('.');
        var mimetype = (dotoffset == -1) ? 'text/plain':
          {
            '.html' : 'text/html',
            '.ico' : 'image/x-icon',
            '.jpg' : 'image/jpeg',
            '.png' : 'image/png',
            '.gif' : 'image/gif',
            '.css' : 'text/css',
            '.js' : 'text/javascript'
          }[request.url.substr(dotoffset)];
          response.writeHead(200,{"Content-Type":mimetype})
          response.end(data)
          console.log( request.url, mimetype );
          return;
      } else {
          console.log ('file not found: ' + request.url);
          response.writeHead(404, "Not Found");
          response.end();
          return;
      }});
});

// Start server
app.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
});