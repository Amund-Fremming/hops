import type { ReactNode } from "react";
import { createContext, useContext, useState } from "react";

type Method = "phone" | "email";

interface SignupState {
  method: Method;
  identifier: string;
  firstName: string;
  lastName: string;
  password: string;
}

interface SignupContextValue extends SignupState {
  setMethod: (m: Method) => void;
  setIdentifier: (v: string) => void;
  setFirstName: (v: string) => void;
  setLastName: (v: string) => void;
  setPassword: (v: string) => void;
  reset: () => void;
}

const initial: SignupState = {
  method: "phone",
  identifier: "",
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
