const express = require('express');
const fs = require('fs');
const child_process = require('child_process');

const app = express();
const PORT = process.env.PORT || 80;

app.use(express.json());

app.post("/move", (req, res) => {
    let body = req.body;
    console.log(body);
    child_process.execSync(`python ../chess_engine.py find_best_move_eval ${body.fen} ${body.color} ${body.depth}`, (error, stdout, stderr) => {
        if (error) {
            console.error(`Error executing move: ${error}`);
            res.status(500).send("Internal Server Error");
            return;
        }
        console.log(stdout);
        res.send(stdout);
        return;
    });
    res.end();
})

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