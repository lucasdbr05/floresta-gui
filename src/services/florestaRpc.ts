import { invoke } from "@tauri-apps/api/core";
import type { FlorestaConfig, BlockchainInfo } from "../types/floresta";

/**
 * Bindings for floresta-rpc
 */
export class FlorestaRpc {
  // Connect to a running florestad instance
  async connectNode(config: FlorestaConfig): Promise<string> {
    return invoke<string>("connect_node", { config });
  }

  //Disconnect from the florestad instance
  async disconnectNode(): Promise<string> {
    return invoke<string>("disconnect_node");
  }

  //Get the current node connection configuration
  async getNodeConfig(): Promise<FlorestaConfig | null> {
    return invoke<FlorestaConfig | null>("get_node_config");
  }

  //Get general blockchain information (height, ibd status, etc.)
  async getBlockchainInfo(): Promise<BlockchainInfo> {
    return invoke<BlockchainInfo>("get_blockchain_info");
  }

  //Get the current block count / height
  async getBlockCount(): Promise<number> {
    return invoke<number>("get_block_count");
  }

  //Get the hash of the best (tip) block
  async getBestBlockHash(): Promise<string> {
    return invoke<string>("get_best_block_hash");
  }

  //Get the block hash at a given height
  async getBlockHash(height: number): Promise<string> {
    return invoke<string>("get_block_hash", { height });
  }

  //Get the block header for a given hash
  async getBlockHeader(hash: string): Promise<Record<string, unknown>> {
    return invoke<Record<string, unknown>>("get_block_header", { hash });
  }
}

export const florestaRpc = new FlorestaRpc();
