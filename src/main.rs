use plotters::prelude::*;

#[derive(Debug)]
struct Graph<const N: usize> {
    V: [usize; N],               // массив вершин (фиксированный)
    E: Vec<(usize, usize, u64)>, // рёбра (u, v, вес)
}

impl<const N: usize> Graph<N> {
    fn new(V: [usize; N]) -> Self {
        Graph { V, E: Vec::new() }
    }
    fn get_sum(&mut self) -> u64 {
        let mut sum: u64 = 0;
        for (_, _, w) in &self.E {
            sum += w;
        }
        sum
    }

    fn add_edge(&mut self, u: usize, v: usize, w: u64) {
        self.E.push((u, v, w));
    }

    fn adjacency_list(&self) -> Vec<Vec<(usize, u64)>> {
        let mut adj = vec![vec![]; N]; // инициализируем N пустых векторов

        for &(u, v, w) in &self.E {
            adj[u].push((v, w));
            adj[v].push((u, w)); // если грав неориентрованный
        }
        adj
    }

    fn is_cycle(&self) -> bool {
        let adj = self.adjacency_list(); // вычисляем один раз
        let mut visited = vec![false; N];

        for v in 0..N {
            if !visited[v] {
                if self.dfs_cycle(v, N, &mut visited, &adj) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs_cycle(
        &self,
        v: usize,
        parent: usize,
        visited: &mut [bool],
        adj: &[Vec<(usize, u64)>],
    ) -> bool {
        visited[v] = true;

        for &(neighbor, _weight) in &adj[v] {
            if !visited[neighbor] {
                // Рекурсивно обходим соседа, текущая вершина становится родителем
                if self.dfs_cycle(neighbor, v, visited, adj) {
                    return true;
                }
            } else if neighbor != parent {
                // Сосед уже посещён и это не родитель -> цикл
                return true;
            }
        }
        false
    }

    fn prima(&mut self) -> Graph<N> {
        let mut tmp_graph = Graph::<N>::new(self.V);
        let mut is_in_v_i = [false; N];
        let mut e_copy_sort: Vec<(usize, usize, u64)> = self.E.clone();
        e_copy_sort.sort_by_key(|&(_, _, w)| w);

        // Добавляем первую вершину (с которой начнём)
        is_in_v_i[0] = true;
        let mut edges_count = 0;

        while edges_count < N - 1 {
            // Ищем минимальное ребро, инцидентное дереву
            let mut min_edge = None;
            let mut min_weight = u64::MAX;

            for &(v_start, v_end, weight) in &e_copy_sort {
                let start_in = is_in_v_i[v_start];
                let end_in = is_in_v_i[v_end];

                // Ровно один конец в дереве
                if (start_in ^ end_in) && (weight < min_weight) {
                    min_weight = weight;
                    min_edge = Some((v_start, v_end, weight));
                }
            }

            // Добавляем найденное ребро
            if let Some((v_start, v_end, weight)) = min_edge {
                tmp_graph.add_edge(v_start, v_end, weight);

                // Отмечаем новую вершину
                if !is_in_v_i[v_start] {
                    is_in_v_i[v_start] = true;
                }
                if !is_in_v_i[v_end] {
                    is_in_v_i[v_end] = true;
                }

                edges_count += 1;
            } else {
                break; // Граф несвязный
            }
        }

        tmp_graph
    }

    fn prima_kraskal(&mut self) -> Graph<N> {
        let mut tmp_graph = Graph::<N>::new(self.V);
        let mut e_copy_sort: Vec<(usize, usize, u64)> = self.E.clone();
        e_copy_sort.sort_by_key(|&(_, _, w)| w);
        for a in e_copy_sort {
            tmp_graph.E.push(a);
            if tmp_graph.is_cycle() {
                tmp_graph.E.pop();
            }
        }
        tmp_graph
    }

    fn draw_graph(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let adj_list = self.adjacency_list();
        let n = adj_list.len();
        let size = 400.0;
        let radius = 160.0;
        let center = size / 2.0;

        let positions: Vec<(f64, f64)> = (0..n)
            .map(|i| {
                let angle = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
                let (x, y) = (center + radius * angle.cos(), center + radius * angle.sin());
                (x, y)
            })
            .collect();

        let root = BitMapBackend::new(filename, (400, 400)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .build_cartesian_2d(0f64..size, 0f64..size)?;

        // Рисуем рёбра с подписями весов
        for (i, neighbors) in adj_list.iter().enumerate() {
            for &(j, weight) in neighbors {
                if i < j {
                    // Рисуем линию
                    chart.draw_series(LineSeries::new(vec![positions[i], positions[j]], &BLACK))?;

                    // Вычисляем середину ребра
                    let (x1, y1) = positions[i];
                    let (x2, y2) = positions[j];
                    let mid_x = (x1 + x2) / 2.0;
                    let mid_y = (y1 + y2) / 2.0;

                    // Рисуем вес ребра
                    chart.draw_series(std::iter::once(Text::new(
                        format!("{}", weight),
                        (mid_x, mid_y),
                        ("sans-serif", 12).into_font().color(&BLUE),
                    )))?;
                }
            }
        }

        // Рисуем вершины и подписи
        for (i, &(x, y)) in positions.iter().enumerate() {
            chart.draw_series(std::iter::once(Circle::new((x, y), 10, RED.filled())))?;
            chart.draw_series(std::iter::once(Text::new(
                i.to_string(),
                (x - 5.0, y + 3.0),
                ("sans-serif", 15).into_font().color(&BLACK),
            )))?;
        }

        root.present()?;
        Ok(())
    }
}

fn main() {
    let vertices = [0, 1, 2, 3, 4];
    let mut graph = Graph::<5>::new(vertices);

    // Рёбра: u, v, вес
    graph.add_edge(0, 1, 2);
    graph.add_edge(0, 2, 3);
    graph.add_edge(0, 3, 1);
    graph.add_edge(0, 4, 4);

    graph.add_edge(1, 2, 5);
    graph.add_edge(1, 3, 2);
    graph.add_edge(1, 4, 6);

    graph.add_edge(2, 3, 7);
    graph.add_edge(2, 4, 2);

    graph.add_edge(3, 4, 3);
    graph.draw_graph("graph.png");

    let mut min_ostov = graph.prima_kraskal();
    min_ostov.draw_graph("prima_kraskal.png");

    let mut prim = graph.prima();
    prim.draw_graph("prima.png");

    println!("prima_sum: {}", prim.get_sum());
    println!("prima_kraskal_sum:{}", min_ostov.get_sum());
}
