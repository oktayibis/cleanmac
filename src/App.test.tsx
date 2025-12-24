import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import App from "./App";

// Invoke is already mocked in setup.ts, but we need to type it to mock return values
const mockedInvoke = vi.mocked(invoke);

describe("App", () => {
  it("renders the sidebar and main content", async () => {
    render(<App />);
    
    expect(await screen.findByText("CleanMac")).toBeInTheDocument();
    expect((await screen.findAllByText("Dashboard")).length).toBeGreaterThan(0);
    expect(await screen.findByText("System Cache")).toBeInTheDocument();
    expect(await screen.findByText("Large Files")).toBeInTheDocument();
  });

  it("handles greeting user", async () => {
    mockedInvoke.mockResolvedValue("Hello, Tester!");
    
    render(<App />);
    
    const input = screen.getByPlaceholderText("Enter your name...");
    const button = screen.getByText("Greet");
    
    fireEvent.change(input, { target: { value: "Tester" } });
    fireEvent.click(button);
    
    await waitFor(() => {
      expect(mockedInvoke).toHaveBeenCalledWith("greet", { name: "Tester" });
      expect(screen.getByText("Hello, Tester!")).toBeInTheDocument();
    });
  });

  it("handles empty input greeting", async () => {
    mockedInvoke.mockResolvedValue("Hello, !");
    
    render(<App />);
    
    const button = screen.getByText("Greet");
    fireEvent.click(button);
    
    await waitFor(() => {
      expect(mockedInvoke).toHaveBeenCalledWith("greet", { name: "" });
    });
  });
});
