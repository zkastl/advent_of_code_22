using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Aoc2022
{
    public class Day8
    {
        readonly string FILE_PATH = Environment.CurrentDirectory + "/day8.txt";
        public static void Main(string[] args)
        {
            // Read the file in for the map
            ushort[,] map = new ushort[99,100];
            using (StreamReader sr = new StreamReader(new FileStream(Environment.CurrentDirectory + "/day8.txt", FileMode.Open)))
            {
                if (sr == null)
                    return;

                int row = 0;
                while(!sr.EndOfStream)
                {
                    string line = sr.ReadLine();
                    for(int col = 0; col < line.Length; col++)
                        map[row, col] = ushort.Parse(line[col].ToString());

                    row++;
                }
            }

            // Transform the map into a list of Visibles
            char[,] transform = new char[99, 100] ;
            for(int row = 0; row < 99; row++)
            {
                for(int col = 0; col < 100; col++)
                {
                    transform[row,col] = 'V';
                    // Check left
                    for(int i = col; i > 0; i--)
                    {
                        if(map[row,i] > map[row,col])
                        {
                            transform[row,col] = 'I';
                            break;
                        }
                    }
                    // Check Top
                    for(int i = row; i > 0; i--)
                    {
                        if(map[i,col] > map[row,col])
                        {
                            transform[row,col] = 'I';
                            break;
                        }
                    }
                    // Check Right
                    for(int i = col; i < 100; i++)
                    {
                        if(map[row,i] > map[row,col])
                        {
                            transform[row,col] = 'I';
                            break;
                        }
                    }
                    // Check Down
                    for(int i = row; i < 99; i++)
                    {
                        if(map[i,col] > map[row,col])
                        {
                            transform[row,col] = 'I';
                            break;
                        }
                    }
                }    
            }
        }
    }
}
