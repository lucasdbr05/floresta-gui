export interface FlorestaConfig {
  network: string;
  rpc_host: string;
  rpc_port: number;
  rpc_user: string | null;
  rpc_password: string | null;
}

export interface BlockchainInfo {
  best_block_hash: string;
  height: number;
  ibd: boolean;
  validated: number;
  latest_work: string;
  latest_block_time: number;
  leaf_count: number;
  root_count: number;
  root_hashes: string[];
  chain: string;
  difficulty: number;
  progress: number;
}
