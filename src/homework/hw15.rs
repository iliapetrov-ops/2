<!DOCTYPE html>
<html lang="uk">
<head>
  <meta charset="UTF-8">
  <title>Криптарифм: muxa × xa = slon</title>
  <style>
    pre {
      font-family: monospace;
      font-size: 140px;
      background: #f4f4f4;
      padding: 100px;
      margin: 100px;
    }
  </style>
</head>
<body>
<pre id="output"></pre>
<script>
  function permutations(arr, k) {
    if (k === 0) return [[]];
    let result = [];
    for (let i = 0; i < arr.length; i++) {
      let rest = [...arr.slice(0, i), ...arr.slice(i + 1)];
      let perms = permutations(rest, k - 1);
      for (let perm of perms) {
        result.push([arr[i], ...perm]);
      }
    }
    return result;
  }
  function solveCryptarithm() {
    const digits = [0,1,2,3,4,5,6,7,8,9];
    const solutions = [];
    let allPerms = permutations(digits, 8);
    console.log(`Всього перестановок: ${allPerms.length}`);
    for (let perm of allPerms) {
      const [m, u, x, a, s, l, o, n] = perm;
      if (m === 0 || s === 0) continue;
      const top = 1000*m + 100*u + 10*x + a;
      const multiplier = 10*x + a;
      const product = top * multiplier;

      if (product < 1000 || product > 9999) continue;

      const s2 = Math.floor(product / 1000);
      const l2 = Math.floor((product % 1000) / 100);
      const o2 = Math.floor((product % 100) / 10);
      const n2 = product % 10;

      if (s2 === s && l2 === l && o2 === o && n2 === n) {
        solutions.push({ top, multiplier, product, digits: { m,u,x,a,s,l,o,n } });
      }
    }
    return solutions;
  }


  function formatSolution(top, multiplier, product) {
    const topStr = top.toString();
    const multiplierStr = multiplier.toString(); // має дві цифри: tens = x, units = a
    const tens = multiplierStr[0];
    const units = multiplierStr[1];
    const prodStr = product.toString().padStart(4, '0');


    let lines = [];
    lines.push(`  ${topStr}`);
    lines.push(`${tens}        ${units}`);
    lines.push(`  ------`);
    lines.push(`    ${prodStr}`);
    return lines.join('\n');
  }


  function run() {
    const solutions = solveCryptarithm();
    let output = `Знайдено розв'язків: ${solutions.length}\n\n`;
    if (solutions.length === 0) {
      output += "Жодного розв'язку не знайдено.";
    } else {
      solutions.forEach((sol, idx) => {
        output += `Розв'язок ${idx+1}:\n`;
        output += formatSolution(sol.top, sol.multiplier, sol.product);
        output += `\nПеревірка: ${sol.top} × ${sol.multiplier} = ${sol.product}\n`;
        output += `Букви: m=${sol.digits.m}, u=${sol.digits.u}, x=${sol.digits.x}, a=${sol.digits.a}, `;
        output += `s=${sol.digits.s}, l=${sol.digits.l}, o=${sol.digits.o}, n=${sol.digits.n}\n\n`;
      });
    }
    
    console.log(output);
    document.getElementById('output').textContent = output;
  }

  run();
</script>
</body>
</html>
