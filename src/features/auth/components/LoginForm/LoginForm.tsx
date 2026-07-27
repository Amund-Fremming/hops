import { useState } from "react";
import { TextInput, View } from "react-native";

import { styles } from "./styles";

interface LoginFormProps {
  onSubmit: (identifier: string, password: string) => void;
}

export function LoginForm({ onSubmit }: LoginFormProps) {
  const [identifier, setIdentifier] = useState("");
  const [password, setPassword] = useState("");

  return (
    <View style={styles.container}>
      <TextInput
        value={identifier}
        onChangeText={setIdentifier}
        placeholder="Email or phone"
        autoCapitalize="none"
        keyboardType="email-address"
      />
      <TextInput
        value={password}
        onChangeText={setPassword}
        placeholder="Password"
        secureTextEntry
      />
    </View>
  );
}
