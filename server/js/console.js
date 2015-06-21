var VeniceConsole = function(inputId, displayId) {
  this.text = '';
  this.input = $(inputId);
  this.display = $(displayId);

  this.onCommand = function(command) {};

  this.addSystemMessage = function(message) {
    this.text += '<p>System: ' + message + '</p>';
    this.updateDisplay();
  }

  this.addCommand = function (command) {
    this.text += '<p>Player: ' + command + '</p>';
    this.updateDisplay();
    this.display.scrollTop(this.display.prop("scrollHeight"));
    this.input.val('');
  }

  this.updateDisplay = function() {
    this.display.html(this.text);
  }

  var self = this;
  this.input.keypress(function (event) {
    if (event.which == 13) {
      var command = self.input.val();
      self.addCommand(command);
      self.onCommand(command);
    }
  });

  this.updateDisplay();
};

(function() {

  var c = new VeniceConsole(
    '#console_input',
    '#console_display');
  c.addSystemMessage("Welcome to Venice - A Game of Auctions!");

  var ws = new WebSocket("ws://localhost:8000/echo");
  ws.onopen = function() {
    ws.send("socket open");
  };
  ws.onclose = function(evt) {
    alert("socket closed");
  };
  ws.onmessage = function(msg) {
   c.addSystemMessage(msg.data);
  }

  c.onCommand(function(command) {
    ws.send(command);
  });

})();
