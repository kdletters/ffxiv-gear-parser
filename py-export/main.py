#!/usr/bin/env python3
"""
FFXIV Gear Parser - Python Bindings Test Suite
===============================================
This script tests the Python bindings for the FFXIV gear parser.
"""

import json
import sys
import time
from typing import Dict, Any

# Try to import the module
try:
    import ffxiv_gear_parser
except ImportError:
    print("❌ Error: ffxiv_gear_parser module not found")
    print("\nPlease install the module first:")
    print("  cd py-export")
    print("  pip install maturin")
    print("  maturin develop")
    print()
    sys.exit(1)


# ANSI color codes for pretty output
class Colors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'


def print_header(text: str):
    """Print a colored header"""
    print(f"\n{Colors.HEADER}{Colors.BOLD}{'=' * 70}{Colors.ENDC}")
    print(f"{Colors.HEADER}{Colors.BOLD}{text:^70}{Colors.ENDC}")
    print(f"{Colors.HEADER}{Colors.BOLD}{'=' * 70}{Colors.ENDC}\n")


def print_test(name: str):
    """Print test name"""
    print(f"{Colors.OKCYAN}🧪 {name}{Colors.ENDC}")


def print_success(message: str):
    """Print success message"""
    print(f"{Colors.OKGREEN}✅ {message}{Colors.ENDC}")


def print_error(message: str):
    """Print error message"""
    print(f"{Colors.FAIL}❌ {message}{Colors.ENDC}")


def print_info(message: str):
    """Print info message"""
    print(f"{Colors.OKBLUE}ℹ️  {message}{Colors.ENDC}")


def print_warning(message: str):
    """Print warning message"""
    print(f"{Colors.WARNING}⚠️  {message}{Colors.ENDC}")


def test_basic_parse():
    """Test basic parse functionality"""
    print_test("Test 1: Basic Parse Function")

    share_string = "45WGpd4LvOX9v3JkHJrIG8hUEncvc0s0we"

    try:
        start_time = time.time()
        result = ffxiv_gear_parser.parse(share_string)
        elapsed = (time.time() - start_time) * 1000  # Convert to ms

        print_success(f"Parse successful (took {elapsed:.2f}ms)")
        print_info(f"Result type: {result['type']}")

        if result['type'] == 'Gearset':
            data = result['data']
            print_info(f"Job: {data['job']}")
            print_info(f"Level: {data['job_level']}")
            print_info(f"Sync Level: {data['sync_level']}")
            print_info(f"Number of gears: {len(data['gears'])}")

            # Check first gear
            if data['gears']:
                first_gear = data['gears'][0]
                print_info(f"First gear ID: {first_gear['id']}")
                print_info(f"First gear materias: {len(first_gear['materias'])} slots")

        return True
    except Exception as e:
        print_error(f"Parse failed: {e}")
        return False


def test_json_parse():
    """Test JSON parse functionality"""
    print_test("Test 2: JSON Parse Function")

    share_string = "45WGpd4LvOX9v3JkHJrIG8hUEncvc0s0we"

    try:
        start_time = time.time()
        json_result = ffxiv_gear_parser.parse_json(share_string)
        elapsed = (time.time() - start_time) * 1000

        data = json.loads(json_result)

        print_success(f"JSON parse successful (took {elapsed:.2f}ms)")
        print_info(f"Result type: {data['type']}")

        if data['type'] == 'Gearset':
            print_info(f"Job: {data['data']['job']}")
            print_info(f"Level: {data['data']['job_level']}")

        print(f"\n{Colors.BOLD}JSON Output (first 500 chars):{Colors.ENDC}")
        print(json_result[:500] + "..." if len(json_result) > 500 else json_result)

        return True
    except Exception as e:
        print_error(f"JSON parse failed: {e}")
        return False


def test_detailed_output():
    """Test detailed output inspection"""
    print_test("Test 3: Detailed Gearset Inspection")

    share_string = "45WGpd4LvOX9v3JkHJrIG8hUEncvc0s0we"

    try:
        result = ffxiv_gear_parser.parse(share_string)

        if result['type'] != 'Gearset':
            print_warning("Not a gearset type, skipping detailed inspection")
            return True

        data = result['data']
        gears = data['gears']

        print_success(f"Inspecting {len(gears)} gear pieces")

        # Count materias
        total_materias = 0
        for gear in gears:
            total_materias += sum(1 for m in gear['materias'] if m is not None)

        print_info(f"Total materias melded: {total_materias}")

        # Check for custom stats
        custom_stat_gears = sum(1 for g in gears if g['custom_stats'] is not None)
        if custom_stat_gears > 0:
            print_info(f"Gears with custom stats: {custom_stat_gears}")

        # Print first few gears
        print(f"\n{Colors.BOLD}First 3 gears:{Colors.ENDC}")
        for i, gear in enumerate(gears[:3], 1):
            print(f"  {i}. ID: {gear['id']}, Materias: {len([m for m in gear['materias'] if m])}")
            for j, materia in enumerate(gear['materias'], 1):
                if materia:
                    print(f"     Slot {j}: {materia['stat']} Grade {materia['grade']}")

        return True
    except Exception as e:
        print_error(f"Detailed inspection failed: {e}")
        return False


def test_invalid_inputs():
    """Test error handling with invalid inputs"""
    print_test("Test 4: Error Handling")

    invalid_cases = [
        ("", "empty string"),
        ("invalid", "invalid string"),
        ("12345", "short numeric string"),
        ("!@#$%", "special characters"),
        ("x" * 1000, "very long string"),
    ]

    all_passed = True

    for test_input, description in invalid_cases:
        try:
            ffxiv_gear_parser.parse(test_input)
            print_error(f"Expected error for {description}, but got success")
            all_passed = False
        except ValueError as e:
            print_success(f"Correctly raised ValueError for {description}")
            print(f"   Error: {str(e)[:80]}...")
        except Exception as e:
            print_warning(f"Unexpected error type for {description}: {type(e).__name__}")
            all_passed = False

    return all_passed


def test_performance():
    """Test parsing performance"""
    print_test("Test 5: Performance Benchmark")

    share_string = "45WGpd4LvOX9v3JkHJrIG8hUEncvc0s0we"
    iterations = 1000

    try:
        print_info(f"Running {iterations} iterations...")

        start_time = time.time()
        for _ in range(iterations):
            ffxiv_gear_parser.parse(share_string)
        elapsed = time.time() - start_time

        avg_time = (elapsed / iterations) * 1000  # ms
        ops_per_sec = iterations / elapsed

        print_success(f"Performance test completed")
        print_info(f"Total time: {elapsed:.3f}s")
        print_info(f"Average time per parse: {avg_time:.3f}ms")
        print_info(f"Operations per second: {ops_per_sec:.0f} ops/s")

        if avg_time < 1.0:
            print_success("⚡ Excellent performance!")
        elif avg_time < 5.0:
            print_success("✓ Good performance")
        else:
            print_warning("Performance could be improved")

        return True
    except Exception as e:
        print_error(f"Performance test failed: {e}")
        return False


def test_multiple_strings():
    """Test parsing multiple different share strings"""
    print_test("Test 6: Multiple Share Strings")

    # Different share strings (add more real examples if available)
    test_strings = [
        "45WGpd4LvOX9v3JkHJrIG8hUEncvc0s0we",
        # Add more test strings here if you have them
    ]

    passed = 0
    failed = 0

    for i, share_string in enumerate(test_strings, 1):
        try:
            result = ffxiv_gear_parser.parse(share_string)
            print_success(f"String {i}: {result['type']}")
            passed += 1
        except Exception as e:
            print_error(f"String {i} failed: {e}")
            failed += 1

    print_info(f"Results: {passed} passed, {failed} failed")
    return failed == 0


def test_json_consistency():
    """Test that parse and parse_json return consistent data"""
    print_test("Test 7: JSON Consistency Check")

    share_string = "45WGpd4LvOX9v3JkHJrIG8hUEncvc0s0we"

    try:
        result_dict = ffxiv_gear_parser.parse(share_string)
        result_json_str = ffxiv_gear_parser.parse_json(share_string)
        result_json = json.loads(result_json_str)

        # Compare types
        if result_dict['type'] != result_json['type']:
            print_error(f"Type mismatch: {result_dict['type']} vs {result_json['type']}")
            return False

        print_success("Result types match")

        # If it's a gearset, check basic fields
        if result_dict['type'] == 'Gearset':
            dict_data = result_dict['data']
            json_data = result_json['data']

            checks = [
                ('job', dict_data['job'] == json_data['job']),
                ('job_level', dict_data['job_level'] == json_data['job_level']),
                ('sync_level', dict_data['sync_level'] == json_data['sync_level']),
                ('gear_count', len(dict_data['gears']) == len(json_data['gears'])),
            ]

            for field, matches in checks:
                if matches:
                    print_success(f"{field} matches")
                else:
                    print_error(f"{field} mismatch")
                    return False

        print_success("JSON consistency check passed")
        return True
    except Exception as e:
        print_error(f"Consistency check failed: {e}")
        return False


def run_all_tests():
    """Run all tests and report results"""
    print_header("FFXIV Gear Parser - Python Bindings Test Suite")

    tests = [
        ("Basic Parse", test_basic_parse),
        ("JSON Parse", test_json_parse),
        ("Detailed Inspection", test_detailed_output),
        ("Error Handling", test_invalid_inputs),
        ("Performance", test_performance),
        ("Multiple Strings", test_multiple_strings),
        ("JSON Consistency", test_json_consistency),
    ]

    results = []

    for name, test_func in tests:
        try:
            result = test_func()
            results.append((name, result))
        except Exception as e:
            print_error(f"Test '{name}' crashed: {e}")
            results.append((name, False))
        print()  # Add spacing between tests

    # Print summary
    print_header("Test Summary")

    passed = sum(1 for _, result in results if result)
    total = len(results)

    for name, result in results:
        status = f"{Colors.OKGREEN}✅ PASSED{Colors.ENDC}" if result else f"{Colors.FAIL}❌ FAILED{Colors.ENDC}"
        print(f"  {name:.<50} {status}")

    print(f"\n{Colors.BOLD}Final Result: {passed}/{total} tests passed{Colors.ENDC}")

    if passed == total:
        print(f"\n{Colors.OKGREEN}{Colors.BOLD}🎉 All tests passed! 🎉{Colors.ENDC}")
        return 0
    else:
        print(f"\n{Colors.FAIL}{Colors.BOLD}❌ Some tests failed{Colors.ENDC}")
        return 1


def interactive_mode():
    """Interactive mode for testing custom share strings"""
    print_header("Interactive Mode")
    print("Enter share strings to parse (or 'quit' to exit)")
    print()

    while True:
        try:
            share_string = input(f"{Colors.OKCYAN}Share string> {Colors.ENDC}").strip()

            if share_string.lower() in ['quit', 'exit', 'q']:
                print("Goodbye!")
                break

            if not share_string:
                continue

            start_time = time.time()
            result = ffxiv_gear_parser.parse(share_string)
            elapsed = (time.time() - start_time) * 1000

            print(f"\n{Colors.OKGREEN}✅ Parsed successfully (took {elapsed:.2f}ms){Colors.ENDC}")
            print(json.dumps(result, indent=2))
            print()

        except ValueError as e:
            print(f"{Colors.FAIL}❌ Parse error: {e}{Colors.ENDC}\n")
        except KeyboardInterrupt:
            print("\nGoodbye!")
            break
        except Exception as e:
            print(f"{Colors.FAIL}❌ Unexpected error: {e}{Colors.ENDC}\n")


def main():
    """Main entry point"""
    if len(sys.argv) > 1:
        if sys.argv[1] in ['-i', '--interactive']:
            interactive_mode()
            return 0
        elif sys.argv[1] in ['-h', '--help']:
            print("Usage:")
            print("  python main.py           Run all tests")
            print("  python main.py -i        Interactive mode")
            print("  python main.py -h        Show this help")
            return 0

    return run_all_tests()


if __name__ == "__main__":
    sys.exit(main())