# 🎬 Movie Genre Influence Analysis

This project explores genre influence within a large Netflix dataset by modeling movie genres as a graph and applying centrality measures to uncover patterns in genre popularity and connectivity. Written in Rust, this project combines graph theory with data science to provide actionable insights for filmmakers and content analysts.

## 📊 Overview

- **Dataset:** 5,752 movie entries from Netflix (includes genre tags)
- **Goal:** Analyze genre relationships and identify which genres are the most central or influential in the network
- **Method:** 
  - Build an undirected graph where nodes = genres, edges = co-occurrence of genres
  - Use **Breadth-First Search (BFS)** to compute **Degree Centrality** and **Betweenness Centrality**

## 🛠️ Built With

- **Language:** Rust  
- **Libraries/Tools:**  
  - `petgraph` – for graph structure and traversal  
  - `serde`, `csv` – for data parsing  
  - `rayon` – for parallel computation (optional optimization)  

## 📌 Key Findings

- **Comedy** had the highest **degree centrality**, suggesting it’s the most commonly co-listed genre.
- **Family** genre had the highest **betweenness centrality**, acting as a bridge between otherwise disconnected genre groups.
- These insights can guide **genre selection** in content production to maximize audience reach.
