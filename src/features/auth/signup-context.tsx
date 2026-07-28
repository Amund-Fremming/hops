import type { ReactNode } from "react";
import { createContext, useContext, useState } from "react";

import type { ProviderType } from "./types";

interface SignupState {
  method: ProviderType;
  identifier: string;
  otpId: string;
  firstName: string;
  lastName: string;
  password: string;
}

interface SignupContextValue extends SignupState {
  setMethod: (m: ProviderType) => void;
  setIdentifier: (v: string) => void;
  setOtpId: (v: string) => void;
  setFirstName: (v: string) => void;
  setLastName: (v: string) => void;
  setPassword: (v: string) => void;
  reset: () => void;
}

const initial: SignupState = {
  method: "phone",
  identifier: "",
  otpId: "",
  firstName: "",
  lastName: "",
  password: "",
};

const SignupContext = createContext<SignupContextValue | null>(null);

export function SignupProvider({ children }: { children: ReactNode }) {
  const [state, setState] = useState<SignupState>(initial);

  const value: SignupContextValue = {
    ...state,
    setMethod: (method) => setState((s) => ({ ...s, method, identifier: "" })),
    setIdentifier: (identifier) => setState((s) => ({ ...s, identifier })),
    setOtpId: (otpId) => setState((s) => ({ ...s, otpId })),
    setFirstName: (firstName) => setState((s) => ({ ...s, firstName })),
    setLastName: (lastName) => setState((s) => ({ ...s, lastName })),
    setPassword: (password) => setState((s) => ({ ...s, password })),
    reset: () => setState(initial),
  };

  return (
    <SignupContext.Provider value={value}>{children}</SignupContext.Provider>
  );
}

export function useSignup() {
  const ctx = useContext(SignupContext);
  if (!ctx) throw new Error("useSignup must be used within SignupProvider");
  return ctx;
}
