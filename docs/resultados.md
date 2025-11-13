# Resultados Experimentales

## Planificación de CPU

| Métrica      | Round Robin | SJF    | Mejor |
| ------------ | ----------- | ------ | ----- |
| T. Espera    |     9.4     |  7.6   | SJF   |
| T. Retorno   |     15.8    |  14.0  | SJF   |
| T. Respuesta |     2.5     |  4.8   | RR    |

## Memoria Virtual

| Algoritmo | Fallos | Aciertos | Tasa Aciertos |
| --------- | ------ | -------- | ------------- |
| FIFO      |    5   |     2    |  28.57%       |
| LRU       |    9   |     5    |  35.71%       |

## Planificación de Disco

| Algoritmo | Movimiento | Eficiencia vs FCFS |
| --------- | ---------- | ------------------ |
| FCFS      | 643        | Baseline           |
| SSTF      | 239        | +62.8%             |
| SCAN      | 302        | +53.0%             |