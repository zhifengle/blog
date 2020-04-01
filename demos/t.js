var TemplateEngine = function (html, options) {
  var re = /<%([^%>]+)?%>/g, reExp = /(^( )?(if|for|else|switch|case|break|{|}))(.*)?/g,
    code = 'var r=[];\n',
    cursor = 0, match;
  var add = function (line, js) {
    js ? (code += line.match(reExp) ? line + '\n' : 'r.push(' + line + ');\n') :
      (code += line != '' ? 'r.push("' + line.replace(/"/g, '\\"') + '");\n' : '');
    return add;
  }
  while (match = re.exec(html)) {
    add(html.slice(cursor, match.index))(match[1], true);
    cursor = match.index + match[0].length;
  }
  add(html.slice(cursor, html.length));
  code = (code + 'return r.join("");').replace(/[\r\t\n]/g, ' ');
  return new Function(code).apply(options)
}

var template =
  'My skills:' +
  '<%if(this.showSkills) {%>' +
  '<%for(var index in this.skills) {%>' +
  '<%this.skills[index]%>' +
  '<%}%>' +
  '<%} else {%>' +
  'none' +
  '<%}%>';
console.log(TemplateEngine(template, {
  skills: ["js", "html", "css"],
  showSkills: true
}));
