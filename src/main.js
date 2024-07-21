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

    // Limpiar resultados anteriores
    window.error.textContent = '';
    window.eulerOutput.innerHTML = '';
    window.eulerMejoradoOutput.innerHTML = '';
    window.rungeKuttaOutput.innerHTML = '';

    if (result.error) {
      window.error.textContent = `Error: ${result.error}`;
    } else {
      if (result.euler) {
        window.eulerOutput.innerHTML = generateTable(result.euler, 'Método de Euler');
      }
      if (result.euler_mejorado) {
        window.eulerMejoradoOutput.innerHTML = generateTable(result.euler_mejorado, 'Método de Euler Mejorado');
      }
      if (result.runge_kutta) {
        window.rungeKuttaOutput.innerHTML = generateTable(result.runge_kutta, 'Método de Runge-Kutta');
      }
    }
  } catch (error) {
    console.error('Error al invocar el comando:', error);
    window.error.textContent = `Error: ${error.message}`;
  }
});

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

