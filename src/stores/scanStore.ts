import { create } from 'zustand';

interface ScanState {
  isScanning: boolean;
  setScanning: (isScanning: boolean) => void;
}

export const useScanStore = create<ScanState>((set) => ({
  isScanning: false,
  setScanning: (isScanning) => set({ isScanning }),
}));
