/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package day12;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class JSONDeepAddTest {
	@Test
	void canDeepAddJSON() {
		assertTrue(JSONDeepAdd.add("[1,2,3]") == 6);
		assertTrue(JSONDeepAdd.add("{\"a\":2,\"b\":4}") == 6);
		assertTrue(JSONDeepAdd.add("[[3]]") == 3);
		assertTrue(JSONDeepAdd.add("{\"a\":{\"b\":4},\"c\":-1}") == 3);
		assertTrue(JSONDeepAdd.add("{\"a\":[-1,1]}") == 0);
		assertTrue(JSONDeepAdd.add("[]") == 0);
		assertTrue(JSONDeepAdd.add("{}") == 0);
	}
}