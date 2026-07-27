import { useState } from "react";
import { TextInput, View } from "react-native";

import { styles } from "./styles";

interface SignupFormProps {
  onSubmit: (data: { name: string; email: string; password: string }) => void;
}

export function SignupForm({ onSubmit }: SignupFormProps) {
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  return (
    <View style={styles.container}>
      <TextInput
        value={name}
        onChangeText={setName}
        placeholder="Name"
        autoCapitalize="words"
      />
      <TextInput
        value={email}
        onChangeText={setEmail}
        placeholder="Email"
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
