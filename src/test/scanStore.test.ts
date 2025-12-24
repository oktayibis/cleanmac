import { describe, it, expect } from 'vitest';
import { useScanStore } from '../stores/scanStore';
import { act } from 'react';

describe('useScanStore', () => {
  it('should have initial state', () => {
    const state = useScanStore.getState();
    expect(state.isScanning).toBe(false);
  });

  it('should update scanning state', () => {
    act(() => {
      useScanStore.getState().setScanning(true);
    });
    expect(useScanStore.getState().isScanning).toBe(true);
  });
});
