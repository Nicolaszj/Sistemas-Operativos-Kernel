#!/usr/bin/env python3
"""
Script para generar gráficos comparativos de algoritmos de memoria
Parte del proyecto Kernel Simulation - Sistemas Operativos
"""

import matplotlib.pyplot as plt
import numpy as np

def plot_page_faults_vs_frames():
    """
    Gráfico: Fallos de página vs. Número de marcos
    Compara FIFO y LRU
    """
    # Datos experimentales (ejecutar simulaciones para obtener datos reales)
    frames = np.array([2, 3, 4, 5, 6, 7, 8])
    
    # Ejemplo con secuencia: 1,2,3,4,1,2,5,1,2,3,4,5 (12 accesos)
    fifo_faults = np.array([10, 9, 10, 5, 5, 4, 4])  # Anomalía de Belady en 4!
    lru_faults = np.array([10, 9, 8, 6, 5, 4, 4])    # LRU no tiene anomalía
    
    plt.figure(figsize=(10, 6))
    plt.plot(frames, fifo_faults, marker='o', linewidth=2, markersize=8, label='FIFO')
    plt.plot(frames, lru_faults, marker='s', linewidth=2, markersize=8, label='LRU')
    
    plt.xlabel('Número de Marcos de Memoria', fontsize=12)
    plt.ylabel('Fallos de Página', fontsize=12)
    plt.title('Comparativa: Fallos de Página vs. Número de Marcos\n(Secuencia: 1,2,3,4,1,2,5,1,2,3,4,5)', 
              fontsize=14, fontweight='bold')
    plt.grid(True, alpha=0.3)
    plt.legend(fontsize=11)
    
    # Marcar anomalía de Belady
    plt.annotate('Anomalía de Belady\n(3→4 marcos: +1 fallo)', 
                 xy=(4, 10), xytext=(5, 10.5),
                 arrowprops=dict(arrowstyle='->', color='red', lw=2),
                 fontsize=10, color='red', fontweight='bold')
    
    plt.tight_layout()
    plt.savefig('mem_fallos_vs_marcos.png', dpi=300)
    print("✅ Gráfico guardado: mem_fallos_vs_marcos.png")
    plt.show()

def plot_hit_rate_comparison():
    """
    Gráfico: Tasa de aciertos (Hit Rate) - FIFO vs LRU
    """
    algorithms = ['FIFO', 'LRU', 'Working Set\n(avanzado)']
    hit_rates = [25.0, 58.3, 75.0]  # Porcentajes
    colors = ['#ff6b6b', '#4ecdc4', '#45b7d1']
    
    plt.figure(figsize=(10, 6))
    bars = plt.bar(algorithms, hit_rates, color=colors, alpha=0.8, edgecolor='black', linewidth=1.5)
    
    # Agregar valores sobre las barras
    for bar, rate in zip(bars, hit_rates):
        height = bar.get_height()
        plt.text(bar.get_x() + bar.get_width()/2., height,
                f'{rate:.1f}%',
                ha='center', va='bottom', fontsize=14, fontweight='bold')
    
    plt.ylabel('Tasa de Aciertos (%)', fontsize=12)
    plt.title('Comparativa de Tasa de Aciertos por Algoritmo\n(Secuencia con Localidad Temporal)', 
              fontsize=14, fontweight='bold')
    plt.ylim(0, 100)
    plt.grid(axis='y', alpha=0.3)
    
    plt.tight_layout()
    plt.savefig('mem_hit_rate.png', dpi=300)
    print("✅ Gráfico guardado: mem_hit_rate.png")
    plt.show()

def plot_disk_movement():
    """
    Gráfico: Movimiento total del cabezal - Comparativa de algoritmos de disco
    """
    algorithms = ['FCFS', 'SSTF', 'SCAN']
    movements = [643, 239, 302]  # Cilindros
    colors = ['#e74c3c', '#2ecc71', '#3498db']
    
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 6))
    
    # Gráfico de barras
    bars = ax1.bar(algorithms, movements, color=colors, alpha=0.8, edgecolor='black', linewidth=1.5)
    for bar, mov in zip(bars, movements):
        height = bar.get_height()
        ax1.text(bar.get_x() + bar.get_width()/2., height,
                f'{mov}\ncilindros',
                ha='center', va='bottom', fontsize=12, fontweight='bold')
    
    ax1.set_ylabel('Movimiento Total (cilindros)', fontsize=12)
    ax1.set_title('Movimiento Total del Cabezal', fontsize=13, fontweight='bold')
    ax1.grid(axis='y', alpha=0.3)
    
    # Gráfico de mejora porcentual vs FCFS
    baseline = movements[0]
    improvements = [(baseline - m) / baseline * 100 for m in movements]
    
    bars2 = ax2.bar(algorithms, improvements, color=colors, alpha=0.8, edgecolor='black', linewidth=1.5)
    for bar, imp in zip(bars2, improvements):
        height = bar.get_height()
        ax2.text(bar.get_x() + bar.get_width()/2., height,
                f'{imp:+.1f}%',
                ha='center', va='bottom' if imp >= 0 else 'top', 
                fontsize=12, fontweight='bold')
    
    ax2.set_ylabel('Mejora vs FCFS (%)', fontsize=12)
    ax2.set_title('Eficiencia Relativa', fontsize=13, fontweight='bold')
    ax2.axhline(y=0, color='black', linestyle='--', alpha=0.5)
    ax2.grid(axis='y', alpha=0.3)
    
    plt.suptitle('Comparativa de Algoritmos de Planificación de Disco\n(Secuencia: 98,183,37,122,14,124,65,67 desde pos. 50)', 
                 fontsize=14, fontweight='bold', y=1.02)
    plt.tight_layout()
    plt.savefig('disk_comparativa.png', dpi=300)
    print("✅ Gráfico guardado: disk_comparativa.png")
    plt.show()

def plot_scheduler_metrics():
    """
    Gráfico: Métricas de scheduling - Round Robin vs SJF
    """
    metrics = ['Tiempo de\nEspera', 'Tiempo de\nRetorno', 'Tiempo de\nRespuesta']
    rr_values = [9.4, 15.8, 2.5]
    sjf_values = [7.6, 14.0, 4.8]
    
    x = np.arange(len(metrics))
    width = 0.35
    
    fig, ax = plt.subplots(figsize=(12, 6))
    bars1 = ax.bar(x - width/2, rr_values, width, label='Round Robin (Q=3)', 
                   color='#9b59b6', alpha=0.8, edgecolor='black')
    bars2 = ax.bar(x + width/2, sjf_values, width, label='SJF', 
                   color='#e67e22', alpha=0.8, edgecolor='black')
    
    # Agregar valores
    for bars in [bars1, bars2]:
        for bar in bars:
            height = bar.get_height()
            ax.text(bar.get_x() + bar.get_width()/2., height,
                    f'{height:.1f}',
                    ha='center', va='bottom', fontsize=11, fontweight='bold')
    
    ax.set_ylabel('Unidades de Tiempo', fontsize=12)
    ax.set_title('Comparativa de Métricas de Scheduling\n(Escenario CPU-bound: 5 procesos)', 
                 fontsize=14, fontweight='bold')
    ax.set_xticks(x)
    ax.set_xticklabels(metrics, fontsize=11)
    ax.legend(fontsize=11)
    ax.grid(axis='y', alpha=0.3)
    
    # Resaltar mejor algoritmo por métrica
    ax.text(0, max(rr_values[0], sjf_values[0]) + 1, 
            '✓ SJF mejor', ha='center', fontsize=10, color='green', fontweight='bold')
    ax.text(1, max(rr_values[1], sjf_values[1]) + 1, 
            '✓ SJF mejor', ha='center', fontsize=10, color='green', fontweight='bold')
    ax.text(2, max(rr_values[2], sjf_values[2]) + 1, 
            '✓ RR mejor', ha='center', fontsize=10, color='green', fontweight='bold')
    
    plt.tight_layout()
    plt.savefig('scheduler_comparativa.png', dpi=300)
    print("✅ Gráfico guardado: scheduler_comparativa.png")
    plt.show()

def main():
    """Generar todos los gráficos"""
    print("\n╔════════════════════════════════════════════════════════╗")
    print("║  Generador de Gráficos - Kernel Simulation            ║")
    print("╚════════════════════════════════════════════════════════╝\n")
    
    print("Generando gráficos...\n")
    
    try:
        print("[1/4] Fallos de página vs. Marcos...")
        plot_page_faults_vs_frames()
        
        print("\n[2/4] Tasa de aciertos (Hit Rate)...")
        plot_hit_rate_comparison()
        
        print("\n[3/4] Planificación de disco...")
        plot_disk_movement()
        
        print("\n[4/4] Métricas de scheduling...")
        plot_scheduler_metrics()
        
        print("\n✅ Todos los gráficos generados exitosamente!")
        print("\nArchivos creados:")
        print("  - mem_fallos_vs_marcos.png")
        print("  - mem_hit_rate.png")
        print("  - disk_comparativa.png")
        print("  - scheduler_comparativa.png")
        
    except Exception as e:
        print(f"\n❌ Error generando gráficos: {e}")
        raise

if __name__ == "__main__":
    main()
