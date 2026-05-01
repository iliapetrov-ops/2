<!DOCTYPE html>
<html lang="uk">
<head>
  <meta charset="UTF-8">
  <title>Зміна регістру</title>
</head>
<body>
<pre id="result" style="font-family: monospace; font-size: 160px; background: #f4f4f4; padding: 100px;"></pre>
<script>
  const UPPER_STRING = "ПРИВІТ, СВІТ!";
  const LOWER_STRING = "привіт, світ!";
  
  function toUpperCaseCustom(str) {
    return str.split('').map(char => {
      const code = char.charCodeAt(0);
      if (code >= 97 && code <= 122) { // a-z
        return String.fromCharCode(code - 32);
      } else if (code >= 1072 && code <= 1103) { 
        return String.fromCharCode(code - 32);
      } else if (code === 1104) { // 'є' -> 'Є'
        return String.fromCharCode(1028);
      } else if (code === 1105) { // 'ї' -> 'Ї'
        return String.fromCharCode(1031);
      }
      return char;
    }).join('');
  }

  
  function toLowerCaseCustom(str) {
    return str.split('').map(char => {
      const code = char.charCodeAt(0);
      if (code >= 65 && code <= 90) {
        return String.fromCharCode(code + 32);
      } else if (code >= 1040 && code <= 1071) { 
        return String.fromCharCode(code + 32);
      } else if (code === 1028) { // 'Є' -> 'є'
        return String.fromCharCode(1104);
      } else if (code === 1031) { // 'Ї' -> 'ї'
        return String.fromCharCode(1105);
      }
      return char;
    }).join('');
  }

  
  const resultUpperToLower = toLowerCaseCustom(UPPER_STRING);
  const resultLowerToUpper = toUpperCaseCustom(LOWER_STRING);

  const output = `Оригінал (верхній): "${UPPER_STRING}"\nПеретворено у нижній: "${resultUpperToLower}"\n\n` +
          `Оригінал (нижній): "${LOWER_STRING}"\nПеретворено у верхній: "${resultLowerToUpper}"`;
  console.log(output);
  document.getElementById("result").textContent = output;
</script>
</body>
</html>
