const { invoke } = window.__TAURI__.tauri;

window.calcularButton.addEventListener('click', async () => {
  const func = window.funcLbl.value;
  const x0 = window.xLbl.value;
  const y0 = window.yLbl.value;
  const h = window.hLbl.value;
  const x_final = window.xFLbl.value;

  if (!func || !x0 || !y0 || !h || !x_final) {
    window.error.textContent = 'Todos los campos son obligatorios.';
    return;
  }

  try {
        //Bug raro que me solicita si o si un atributo xFinal para la funcion que sinceramente
        //no tengo idea de dónde sale, pero si no no jala xd
    const result = await invoke('calcular', { func, x0, y0, h, x_final, xFinal: x_final });
    const json = JSON.parse(result);
    
    
    // Limpiar resultados anteriores
    window.error.textContent = '';
    window.eulerOutput.innerHTML = '';
    window.eulerMejoradoOutput.innerHTML = '';
    window.rungeKuttaOutput.innerHTML = '';

    window.eulerOutput.innerHTML = generateTable(json.euler, ["x", "y"], "Método Euler");
    window.eulerMejoradoOutput.innerHTML = generateTable(json.euler_mejorado, ["x", "y", "y_star"], "Método Euler Mejorado");
    window.rungeKuttaOutput.innerHTML = generateTable(json.rk4, ["x", "y", "k1", "k2", "k3", "k4"], "Método RK4");

  } catch (error) {
    console.error('Error al invocar el comando:', error);
    window.error.textContent = `Error: ${error.message}`;
  }
});

function generateTable(data, headers, title){
  let table = `<h3>${title}</h3><table border="1" cellpadding="5" cellspacing="0"><thead><tr>`;
  headers.forEach(header => {
    table += `<th>${header}</th>`;
  });
  table += `</tr></thead><tbody>`;

  data.forEach(item => {
    table += `<tr>`;
    headers.forEach(header => {
      table += `<td>${item[header]}</td>`;
    });
    table += `</tr>`;
  });

  table += `</tbody></table>`;
  return table;
}

/*
// Función para generar una tabla a partir de los datos
function generateTable(data, title) {
  let tableHtml = `<h3>${title}</h3><table border="1" cellpadding="5" cellspacing="0"><thead><tr>`;

  // Asumimos que todos los objetos tienen las mismas claves
  const headers = Object.keys(data[0]);
  headers.forEach(header => {
    tableHtml += `<th>${header}</th>`;
  });
  tableHtml += `</tr></thead><tbody>`;

  data.forEach(item => {
    tableHtml += `<tr>`;
    headers.forEach(header => {
      tableHtml += `<td>${item[header]}</td>`;
    });
    tableHtml += `</tr>`;
  });

  tableHtml += `</tbody></table>`;
  return tableHtml;
}
*/