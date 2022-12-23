using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace Aoc2022.cs_src
{
    public class Day8
    {
        readonly string FILE_PATH = Environment.CurrentDirectory + "/day8.txt";
        public static void Puzzle()
        {
            // Read the file in for the map
            const int ROW_MAX = 99;
            const int COL_MAX = 99;
            ushort[,] map = new ushort[ROW_MAX, COL_MAX];
            using (StreamReader sr = new StreamReader(new FileStream(Environment.CurrentDirectory + "/day8.txt", FileMode.Open)))
            {
                if (sr == null)
                    return;

                int row = 0;
                while (!sr.EndOfStream)
                {
                    string line = sr.ReadLine();
                    for (int col = 0; col < line.Length; col++)
                        map[row, col] = ushort.Parse(line[col].ToString());

                    row++;
                }
            }

            VisibilityMatrix(map, ROW_MAX, COL_MAX);
            Console.WriteLine();
            SenicScore(map, ROW_MAX, COL_MAX);
        }

        public static void VisibilityMatrix(ushort[,] map, int ROW_MAX, int COL_MAX)
        {
            // Transform the map into a list of Visibles
            char[,] transform = new char[ROW_MAX, COL_MAX];
            for (int row = 0; row < ROW_MAX; row++)
            {
                for (int col = 0; col < COL_MAX; col++)
                {
                    bool L = false;
                    bool T = false;
                    bool R = false;
                    bool B = false;

                    transform[row, col] = 'V';

                    // if we're an 'edge' tree', we are always visible
                    if (row == 0 || row == ROW_MAX - 1 || col == 0 || col == COL_MAX - 1)
                    {
                        continue;
                    }

                    // Check left
                    for (int c = 0; c < col; c++)
                    {
                        if (map[row, c] >= map[row, col])
                        {
                            L = true;
                            break;
                        }
                    }

                    // Check Top
                    for (int r = 0; r < row; r++)
                    {
                        if (map[r, col] >= map[row, col])
                        {
                            T = true;
                            break;
                        }
                    }

                    // Check Right
                    for (int c = col + 1; c < COL_MAX; c++)
                    {
                        if (map[row, c] >= map[row, col])
                        {
                            R = true;
                            break;
                        }
                    }

                    // Check Down
                    for (int r = row + 1; r < ROW_MAX; r++)
                    {
                        if (map[r, col] >= map[row, col])
                        {
                            B = true;
                            break;
                        }
                    }

                    if (L && R && T && B)
                        transform[row, col] = 'H';
                }
            }

            int numVis = 0;
            // print the table
            for (int row = 0; row < ROW_MAX; row++)
            {
                for (int col = 0; col < COL_MAX; col++)
                {
                    char val = transform[row, col];
                    if (val == 'V')
                    {
                        Console.Write(transform[row, col]);
                        numVis++;
                    }
                    else
                        Console.Write(' ');
                }
                Console.WriteLine();
            }

            Console.WriteLine("Number of visible trees: " + numVis);
        }

        public static void SenicScore(ushort[,] map, int rowMax, int colMax)
        {
            uint[,] visScoreMat = new uint[rowMax, colMax];
            for (int row = 0; row < rowMax; row++)
            {
                for (int col = 0; col < colMax; col++)
                {
                    uint[] visScore = { 0, 0, 0, 0 };

                    // Check left
                    for (int c = col; c > 0; c--)
                    {
                        visScore[0]++;
                        if (map[row, c - 1] >= map[row, col])
                            break;
                    }

                    // Check Top
                    for (int r = row; r > 0; r--)
                    {
                        visScore[1]++;
                        if (map[r - 1, col] >= map[row, col])
                            break;
                    }

                    // Check Right
                    for (int c = col; c < colMax - 1; c++)
                    {
                        if (map[row, c + 1] >= map[row, col])
                        {
                            visScore[2]++;
                            break;
                        }

                        visScore[2]++;
                    }

                    // Check Down
                    for (int r = row; r < rowMax - 1; r++)
                    {
                        visScore[3]++;
                        if (map[r + 1, col] >= map[row, col])
                            break;
                    }

                    visScoreMat[row, col] = visScore[0] * visScore[1] * visScore[2] * visScore[3];
                }
            }

            uint max = 0;
            // print the table
            for (int row = 0; row < rowMax; row++)
            {
                for (int col = 0; col < colMax; col++)
                {
                    uint val = visScoreMat[row, col];
                    if (val > max)
                        max = val;
                    Console.Write(val + ",");
                }
                Console.WriteLine();
            }

            Console.WriteLine("Max Senic Score: " + max);
        }
    }
}
