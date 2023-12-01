export interface IGraph {
  /**
   * Draw the graph, and return a function that can be used to
   * destroy the graph.
   */
  draw(ref: HTMLElement | null): () => void;
}
