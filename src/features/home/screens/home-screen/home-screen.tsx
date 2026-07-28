import { Pressable, Text, View } from "react-native";

import { useSession } from "@/features/auth";

import { styles } from "./styles";

export function HomeScreen() {
  const { logout } = useSession();

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Home</Text>
      <Pressable onPress={logout} style={styles.logoutButton}>
        <Text style={styles.logoutText}>Logout</Text>
      </Pressable>
    </View>
  );
}
