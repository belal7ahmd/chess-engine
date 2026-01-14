const express = require('express');
const fs = require('fs');
const { spawn } = require('child_process');

const app = express();
const PORT = process.env.PORT || 80;

app.use(express.json());

let engineProcess = spawn("../engine/target/release/engine.exe");

engineProcess.stdout.on('data', (data) => {
    console.log(`Engine Response: ${data}`);
});

// Handle errors from the engine
engineProcess.stderr.on('data', (data) => {
    console.error(`Engine Error: ${data}`);
});

app.post("/move", async (req, res) => {
  const { fen, color, depth } = req.body;
  console.log({ fen, color, depth });
  try {
    engineProcess.stdin.write(JSON.stringify({command: "eval_move", fen: fen, color: color, depth: depth}) + "\n");

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